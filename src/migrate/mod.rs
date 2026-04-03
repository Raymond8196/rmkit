use crate::chip::{get_board_chip_map, get_chip_options};
use crate::keyboard_toml::ProjectInfo;
use crate::version;
use inquire::{Select, Text};
use std::error::Error;
use std::fs;

pub(crate) mod converter;
pub(crate) mod ir;
pub(crate) mod keycode_map;
pub(crate) mod qmk_parser;
pub(crate) mod vial_parser;
pub(crate) mod zmk_parser;

/// Entry point for the `migrate` subcommand
pub(crate) async fn migrate_project(
    from: Option<String>,
    config: Option<String>,
    keymap: Option<String>,
    via_json: Option<String>,
    chip: Option<String>,
    target_dir: Option<String>,
    version: Option<String>,
) -> Result<(), Box<dyn Error>> {
    // 1. Determine source format
    let source = if let Some(f) = from {
        f.to_lowercase()
    } else {
        Select::new(
            "Migrate from which firmware?",
            vec!["vial", "qmk", "zmk"],
        )
        .prompt()?
        .to_string()
    };

    // 2. Parse source config into IR
    let mut ir = match source.as_str() {
        "vial" => {
            let config_path = if let Some(p) = config {
                p
            } else {
                Text::new("Path to vial.json:")
                    .with_default("./vial.json")
                    .prompt()?
            };
            vial_parser::parse_vial_json(&config_path)
                .map_err(|e| format!("Failed to read '{}': {}", config_path, e))?
        }
        "qmk" => {
            let config_path = if let Some(p) = config {
                p
            } else {
                Text::new("Path to QMK info.json or keyboard.json:")
                    .with_default("./keyboard.json")
                    .prompt()?
            };
            let mut ir = qmk_parser::parse_qmk_info_json(&config_path)
                .map_err(|e| format!("Failed to read '{}': {}", config_path, e))?;

            // Parse keymap.json if provided
            let keymap_path = if let Some(p) = keymap {
                Some(p)
            } else {
                let input = Text::new(
                    "Path to keymap.json (export from https://config.qmk.fm, empty to skip):",
                )
                .with_default("")
                .prompt()?;
                if input.is_empty() { None } else { Some(input) }
            };
            if let Some(ref km_path) = keymap_path {
                qmk_parser::parse_qmk_keymap_json(km_path, &mut ir)
                    .map_err(|e| format!("Failed to read '{}': {}", km_path, e))?;
            }

            // Use via.json if provided (for vial.json generation)
            if let Some(ref via_path) = via_json {
                let via_content = fs::read_to_string(via_path)
                    .map_err(|e| format!("Failed to read '{}': {}", via_path, e))?;
                let via_value: serde_json::Value = serde_json::from_str(&via_content)?;
                ir.vial_json_raw = Some(via_value);
            }

            ir
        }
        "zmk" => {
            let keymap_path = if let Some(p) = config {
                p
            } else {
                Text::new("Path to ZMK .keymap file:")
                    .with_default("./corne.keymap")
                    .prompt()?
            };
            let mut ir = zmk_parser::parse_zmk_keymap(&keymap_path)
                .map_err(|e| format!("Failed to read '{}': {}", keymap_path, e))?;

            // Optionally parse .conf file — guess default from .keymap path
            let conf_default = keymap_path
                .strip_suffix(".keymap")
                .map(|base| format!("{}.conf", base))
                .filter(|p| std::path::Path::new(p).exists())
                .unwrap_or_default();
            let conf_path = {
                let input = Text::new("Path to .conf file (leave empty to skip):")
                    .with_default(&conf_default)
                    .prompt()?;
                if input.is_empty() { None } else { Some(input) }
            };
            if let Some(ref conf) = conf_path {
                zmk_parser::parse_zmk_conf(conf, &mut ir)
                    .map_err(|e| format!("Failed to read '{}': {}", conf, e))?;
            }

            ir
        }
        other => {
            return Err(format!(
                "Source '{}' is not yet supported. Currently supported: vial, qmk, zmk",
                other
            )
            .into());
        }
    };

    // 3. Interactive prompts for missing fields
    let default_name = ir
        .name
        .as_deref()
        .or(ir.manufacturer.as_deref())
        .unwrap_or("my_keyboard")
        .replace(" ", "_");
    let project_name = Text::new("Project name:")
        .with_default(&default_name)
        .prompt()?
        .replace(" ", "_");

    let split = if let Some(s) = ir.is_split {
        println!(
            "  Detected keyboard type: {}",
            if s { "split" } else { "normal" }
        );
        s
    } else {
        Select::new("Keyboard type?", vec!["normal", "split"]).prompt()? == "split"
    };
    ir.is_split = Some(split);

    let mut chip_or_board = if let Some(c) = chip {
        c
    } else if let Some(ref hint) = ir.chip_hint {
        let input = Text::new("Target chip (detected hint):")
            .with_default(hint)
            .prompt()?;
        input
    } else {
        Select::new(
            "Choose your microcontroller or board",
            get_chip_options(split),
        )
        .prompt()?
        .to_string()
    };

    let layers = if let Some(l) = ir.layers {
        l
    } else {
        let input = Text::new("Number of layers:").with_default("2").prompt()?;
        input.parse::<u8>()?
    };
    ir.layers = Some(layers);

    // 4. Resolve chip
    let board_chip_map = get_board_chip_map();
    if let Some(c) = board_chip_map.get(chip_or_board.as_str()) {
        chip_or_board = c.to_string();
    }

    let remote_folder = if split {
        format!("{}_split", chip_or_board)
    } else {
        chip_or_board.clone()
    };

    let uf2_key = if chip_or_board.starts_with("stm32") {
        chip_or_board[..7].to_string()
    } else if chip_or_board == "pico_w" {
        "rp2040".to_string()
    } else {
        chip_or_board.clone()
    };

    // 5. Generate keyboard.toml content
    ir.ble_enabled = Some(is_ble_chip(&chip_or_board));
    let keyboard_toml_content = converter::generate_keyboard_toml(&ir, &chip_or_board)?;

    // 6. Generate/prepare vial.json content
    let vial_json_content = converter::generate_vial_json(&ir)?;

    // 7. Set up project directory
    let target_dir_name = if let Some(dir) = target_dir {
        dir
    } else {
        project_name.clone()
    };
    let project_dir = std::env::current_dir()?.join(&target_dir_name);
    fs::create_dir_all(&project_dir)?;

    // 8. Resolve version and download template
    let commit_or_branch = version::resolve_template_version(version.as_deref()).await?;

    let project_info = ProjectInfo {
        project_name: project_name.clone(),
        target_dir: project_dir.clone(),
        remote_folder,
        chip: chip_or_board.clone(),
        uf2_key,
        disabled_default_feature: Vec::new(),
        enabled_feature: Vec::new(),
    };

    crate::download_project_template(&project_info, &commit_or_branch).await?;

    // 9. Write keyboard.toml and vial.json into the project
    fs::write(project_dir.join("keyboard.toml"), keyboard_toml_content)?;
    fs::write(project_dir.join("vial.json"), vial_json_content)?;

    // 10. Post-process (replace placeholders, manage features)
    crate::post_process(project_info)?;

    // 11. Print summary and warnings
    println!();
    if !ir.warnings.is_empty() {
        println!(
            "⚠ {} migration warning(s):",
            ir.warnings.len()
        );
        for w in &ir.warnings {
            println!("  - {}", w);
        }
        println!();
    }

    println!("✅ Migration complete!");
    println!("   Project: {}", project_dir.display());
    println!("   Chip:    {}", chip_or_board);
    if ir.keymap.is_some() {
        println!("   Keymap:  migrated ({} layers)", ir.layers.unwrap_or(0));
    } else {
        println!("   Keymap:  placeholder (fill in manually)");
    }
    println!();
    println!("Next steps:");
    println!("  1. cd {}", project_dir.display());
    println!("  2. Review keyboard.toml — search for TODO to find sections needing your input");
    println!("  3. Fill in matrix pin definitions for your hardware");
    if ir.is_split == Some(true) {
        println!("  4. Uncomment and configure the [split] section");
    }
    println!();
    println!("To build:  cargo build --release");
    Ok(())
}

fn is_ble_chip(chip: &str) -> bool {
    chip.starts_with("nrf")
        || chip.starts_with("esp32")
        || chip == "pico_w"
}
