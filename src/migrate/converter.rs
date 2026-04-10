use super::ir::KeyboardIR;
use std::error::Error;
use std::fmt::Write;

/// Generate keyboard.toml content from a KeyboardIR.
///
/// Fields that could not be extracted from the source are filled with
/// sensible defaults and TODO comments for the user to complete.
pub(crate) fn generate_keyboard_toml(
    ir: &KeyboardIR,
    chip: &str,
) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();

    // [keyboard] section
    let name = ir.name.as_deref().unwrap_or("RMK Keyboard");
    let vid = ir.vendor_id.unwrap_or(0x4C4B);
    let pid = ir.product_id.unwrap_or(0x4643);
    let manufacturer = ir.manufacturer.as_deref().unwrap_or("Migrated");

    writeln!(out, "[keyboard]")?;
    writeln!(out, "name = \"{}\"", name.replace(" ", "_"))?;
    writeln!(out, "product_name = \"{}\"", name)?;
    if ir.vendor_id.is_none() {
        writeln!(out, "# TODO: Set your USB vendor ID")?;
    }
    writeln!(out, "vendor_id = {:#06X}", vid)?;
    if ir.product_id.is_none() {
        writeln!(out, "# TODO: Set your USB product ID")?;
    }
    writeln!(out, "product_id = {:#06X}", pid)?;
    writeln!(out, "manufacturer = \"{}\"", manufacturer)?;
    writeln!(out, "chip = \"{}\"", chip)?;
    writeln!(out)?;

    // [matrix] section
    let rows = ir.rows.unwrap_or(1);
    let cols = ir.cols.unwrap_or(1);

    writeln!(out, "[matrix]")?;
    if let Some(ref pins) = ir.input_pins {
        writeln!(out, "input_pins = {:?}", pins)?;
    } else {
        writeln!(out, "# TODO: Fill in your input (row) pins")?;
        writeln!(
            out,
            "# input_pins = [{}]",
            generate_placeholder_pins(rows as usize, chip)
        )?;
    }
    if let Some(ref pins) = ir.output_pins {
        writeln!(out, "output_pins = {:?}", pins)?;
    } else {
        writeln!(out, "# TODO: Fill in your output (col) pins")?;
        writeln!(
            out,
            "# output_pins = [{}]",
            generate_placeholder_pins(cols as usize, chip)
        )?;
    }
    writeln!(out)?;

    // [layout] section
    let layers = ir.layers.unwrap_or(2) as usize;

    writeln!(out, "[layout]")?;
    writeln!(out, "rows = {}", rows)?;
    writeln!(out, "cols = {}", cols)?;
    writeln!(out, "layers = {}", layers)?;

    writeln!(out, "keymap = [")?;
    if let Some(ref keymap) = ir.keymap {
        for layer in keymap {
            writeln!(out, "    [")?;
            for row in layer {
                let keys: Vec<String> = row.iter().map(|k| format!("\"{}\"", k)).collect();
                writeln!(out, "        [{}],", keys.join(", "))?;
            }
            writeln!(out, "    ],")?;
        }
    } else {
        // Generate default placeholder keymap
        writeln!(out, "    # TODO: Fill in your keymap")?;
        for _layer_idx in 0..layers {
            writeln!(out, "    [")?;
            for _row in 0..rows {
                let keys: Vec<&str> = vec!["\"_\""; cols as usize];
                writeln!(out, "        [{}],", keys.join(", "))?;
            }
            writeln!(out, "    ],")?;
        }
    }
    writeln!(out, "]")?;
    writeln!(out)?;

    // [storage] section
    writeln!(out, "[storage]")?;
    writeln!(out)?;

    // [ble] section
    if ir.ble_enabled == Some(true) {
        writeln!(out, "[ble]")?;
        writeln!(out, "enabled = true")?;
        writeln!(out)?;
    }

    // [split] section hint
    // Split keyboards share the same column count per half; rows are divided.
    if ir.is_split == Some(true) {
        let half_rows = rows / 2;
        writeln!(out, "# TODO: Configure split keyboard settings")?;
        writeln!(out, "# [split]")?;
        writeln!(out, "# connection = \"ble\"")?;
        writeln!(out, "#")?;
        writeln!(out, "# [split.central]")?;
        writeln!(out, "# rows = {}", half_rows)?;
        writeln!(out, "# cols = {}", cols)?;
        writeln!(out, "# row_offset = 0")?;
        writeln!(out, "# col_offset = 0")?;
        writeln!(out, "# ble_addr = [0x18, 0xe2, 0x21, 0x80, 0xc0, 0xc7]")?;
        writeln!(out, "# [split.central.matrix]")?;
        writeln!(
            out,
            "# input_pins = [{}]",
            generate_placeholder_pins(half_rows as usize, chip)
        )?;
        writeln!(
            out,
            "# output_pins = [{}]",
            generate_placeholder_pins(cols as usize, chip)
        )?;
        writeln!(out, "#")?;
        writeln!(out, "# [[split.peripheral]]")?;
        writeln!(out, "# rows = {}", half_rows)?;
        writeln!(out, "# cols = {}", cols)?;
        writeln!(out, "# row_offset = {}", half_rows)?;
        writeln!(out, "# col_offset = 0")?;
        writeln!(out, "# ble_addr = [0x7e, 0xfe, 0x73, 0x9e, 0x66, 0xe3]")?;
        writeln!(out, "# [split.peripheral.matrix]")?;
        writeln!(
            out,
            "# input_pins = [{}]",
            generate_placeholder_pins(half_rows as usize, chip)
        )?;
        writeln!(
            out,
            "# output_pins = [{}]",
            generate_placeholder_pins(cols as usize, chip)
        )?;
        writeln!(out)?;
    }

    Ok(out)
}

/// Generate vial.json content from IR.
///
/// If the IR contains a raw vial.json (e.g., from vial source), return it as-is.
/// Otherwise, construct a minimal vial.json from available IR fields.
pub(crate) fn generate_vial_json(ir: &KeyboardIR) -> Result<String, Box<dyn Error>> {
    if let Some(ref raw) = ir.vial_json_raw {
        return Ok(serde_json::to_string_pretty(raw)?);
    }

    // Build a minimal vial.json from IR fields
    let name = ir.name.as_deref().unwrap_or("RMK Keyboard");
    let vid = ir.vendor_id.unwrap_or(0x4C4B);
    let pid = ir.product_id.unwrap_or(0x4643);
    let rows = ir.rows.unwrap_or(1);
    let cols = ir.cols.unwrap_or(1);

    let mut vial = serde_json::json!({
        "name": name,
        "vendorId": format!("{:#06X}", vid),
        "productId": format!("{:#06X}", pid),
        "lighting": "none",
        "matrix": {
            "rows": rows,
            "cols": cols
        }
    });

    // Physical layout
    if let Some(ref layout) = ir.physical_layout {
        vial["layouts"] = serde_json::json!({ "keymap": layout });
    } else {
        // Generate a simple grid layout
        let mut keymap: Vec<Vec<String>> = Vec::new();
        for r in 0..rows {
            let row: Vec<String> = (0..cols).map(|c| format!("{},{}", r, c)).collect();
            keymap.push(row);
        }
        vial["layouts"] = serde_json::json!({ "keymap": keymap });
    }

    // Custom keycodes
    if let Some(ref custom) = ir.custom_keycodes {
        vial["customKeycodes"] = custom.clone();
    }

    Ok(serde_json::to_string_pretty(&vial)?)
}

/// Generate placeholder pin names based on chip type for TODO comments
fn generate_placeholder_pins(count: usize, chip: &str) -> String {
    let prefix = if chip.starts_with("nrf") {
        "P0_"
    } else if chip.starts_with("stm32") {
        "PA"
    } else if chip.starts_with("rp") || chip == "pico_w" {
        "PIN_"
    } else if chip.starts_with("esp32") {
        "GPIO_"
    } else {
        "PIN_"
    };

    let pins: Vec<String> = (0..count).map(|i| format!("\"{}{}\"", prefix, i)).collect();
    pins.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keyboard_toml_from_ir() {
        let ir = KeyboardIR {
            name: Some("Test KB".into()),
            vendor_id: Some(0xABCD),
            product_id: Some(0x1234),
            rows: Some(2),
            cols: Some(3),
            layers: Some(2),
            ..Default::default()
        };

        let toml = generate_keyboard_toml(&ir, "nrf52840").unwrap();

        assert!(toml.contains("name = \"Test_KB\""));
        assert!(toml.contains("vendor_id = 0xABCD"));
        assert!(toml.contains("product_id = 0x1234"));
        assert!(toml.contains("chip = \"nrf52840\""));
        assert!(toml.contains("rows = 2"));
        assert!(toml.contains("cols = 3"));
        assert!(toml.contains("layers = 2"));
        assert!(toml.contains("# TODO: Fill in your input (row) pins"));
    }

    #[test]
    fn test_generate_keyboard_toml_with_ble_and_split() {
        let ir = KeyboardIR {
            name: Some("Split KB".into()),
            rows: Some(4),
            cols: Some(6),
            layers: Some(3),
            ble_enabled: Some(true),
            is_split: Some(true),
            ..Default::default()
        };

        let toml = generate_keyboard_toml(&ir, "nrf52840").unwrap();

        assert!(toml.contains("[ble]"));
        assert!(toml.contains("enabled = true"));
        assert!(toml.contains("# TODO: Configure split keyboard settings"));
    }

    #[test]
    fn test_generate_keyboard_toml_with_keymap() {
        let keymap = vec![vec![
            vec!["A".into(), "B".into()],
            vec!["C".into(), "D".into()],
        ]];
        let ir = KeyboardIR {
            rows: Some(2),
            cols: Some(2),
            layers: Some(1),
            keymap: Some(keymap),
            ..Default::default()
        };

        let toml = generate_keyboard_toml(&ir, "rp2040").unwrap();
        assert!(toml.contains("\"A\", \"B\""));
        assert!(toml.contains("\"C\", \"D\""));
    }

    #[test]
    fn test_default_keymap_dimensions() {
        let ir = KeyboardIR {
            rows: Some(3),
            cols: Some(4),
            layers: Some(2),
            ..Default::default()
        };

        let toml = generate_keyboard_toml(&ir, "rp2040").unwrap();
        // Should have 2 layers, each with 3 rows of 4 keys
        let layer_count = toml.matches("        [\"_\"").count();
        assert_eq!(layer_count, 6); // 2 layers × 3 rows
    }

    #[test]
    fn test_generate_vial_json_from_raw() {
        let raw = serde_json::json!({"name": "Test", "matrix": {"rows": 2, "cols": 2}});
        let ir = KeyboardIR {
            vial_json_raw: Some(raw.clone()),
            ..Default::default()
        };

        let json_str = generate_vial_json(&ir).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["name"], "Test");
    }

    #[test]
    fn test_generate_vial_json_from_ir() {
        let ir = KeyboardIR {
            name: Some("My Board".into()),
            vendor_id: Some(0xBEEF),
            product_id: Some(0xCAFE),
            rows: Some(2),
            cols: Some(3),
            ..Default::default()
        };

        let json_str = generate_vial_json(&ir).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(parsed["name"], "My Board");
        assert_eq!(parsed["matrix"]["rows"], 2);
        assert_eq!(parsed["matrix"]["cols"], 3);
        assert!(parsed["layouts"]["keymap"].is_array());
    }

    /// Integration test: full pipeline from repo's vial.json → keyboard.toml + vial.json
    #[test]
    fn test_full_pipeline_from_repo_vial() {
        use crate::migrate::vial_parser;

        let repo_vial = include_str!("../../vial.json");
        let mut ir = vial_parser::parse_vial_json_str(repo_vial).unwrap();
        ir.layers = Some(2);
        ir.ble_enabled = Some(true);
        ir.is_split = Some(false);

        // Generate keyboard.toml
        let toml = generate_keyboard_toml(&ir, "nrf52840").unwrap();
        assert!(toml.contains("name = \"HID_Keyboard\""));
        assert!(toml.contains("vendor_id = 0x4C4B"));
        assert!(toml.contains("product_id = 0x4643"));
        assert!(toml.contains("chip = \"nrf52840\""));
        assert!(toml.contains("rows = 4"));
        assert!(toml.contains("cols = 3"));
        assert!(toml.contains("layers = 2"));
        assert!(toml.contains("[ble]"));
        assert!(toml.contains("# TODO: Fill in your input (row) pins"));
        // Should NOT have split section for non-split
        assert!(!toml.contains("# TODO: Configure split keyboard settings"));

        // Generate vial.json (should pass through the raw vial.json)
        let vial_out = generate_vial_json(&ir).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&vial_out).unwrap();
        assert_eq!(parsed["name"], "HID Keyboard");
        assert_eq!(parsed["matrix"]["rows"], 4);
        assert_eq!(parsed["matrix"]["cols"], 3);
        assert_eq!(parsed["customKeycodes"].as_array().unwrap().len(), 12);
    }

    #[test]
    fn test_placeholder_pins() {
        assert_eq!(
            generate_placeholder_pins(3, "nrf52840"),
            "\"P0_0\", \"P0_1\", \"P0_2\""
        );
        assert_eq!(
            generate_placeholder_pins(2, "stm32f303"),
            "\"PA0\", \"PA1\""
        );
        assert_eq!(
            generate_placeholder_pins(2, "rp2040"),
            "\"PIN_0\", \"PIN_1\""
        );
    }
}
