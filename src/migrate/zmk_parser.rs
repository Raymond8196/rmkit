use super::ir::KeyboardIR;
use super::keycode_map;
use std::error::Error;
use std::fs;

/// Parse a ZMK .keymap file into IR.
pub(crate) fn parse_zmk_keymap(path: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_zmk_keymap_str(&content)
}

/// Parse ZMK .keymap content string into IR.
pub(crate) fn parse_zmk_keymap_str(content: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let mut ir = KeyboardIR::default();

    let layers = extract_layers(content);
    if layers.is_empty() {
        return Err("No layers found in .keymap file. Expected `bindings = <...>;` blocks.".into());
    }

    ir.layers = Some(layers.len() as u8);

    // Parse bindings in each layer
    let mut keymap: Vec<Vec<Vec<String>>> = Vec::new();
    let mut max_keys = 0usize;

    for (layer_idx, (_layer_name, bindings_str)) in layers.iter().enumerate() {
        let behaviors = parse_bindings(bindings_str);
        if behaviors.len() > max_keys {
            max_keys = behaviors.len();
        }

        let mut rmk_keys: Vec<String> = Vec::new();
        for behavior in &behaviors {
            match keycode_map::map_zmk_behavior(behavior) {
                Ok(mapped) => rmk_keys.push(mapped),
                Err(warning) => {
                    ir.warnings.push(format!(
                        "Layer {} ({}), key {}: {}",
                        layer_idx,
                        _layer_name,
                        rmk_keys.len(),
                        warning
                    ));
                    rmk_keys.push("_".into());
                }
            }
        }
        keymap.push(vec![rmk_keys]);
    }

    // Guess matrix dimensions: we have a flat key list per layer.
    // Common split keyboards: 42 keys = 6 cols × 7 rows (or various combos).
    // We'll try to use a reasonable default or leave it for the user.
    if max_keys > 0 {
        // Try to find a nice rows×cols factorization
        let (rows, cols) = guess_matrix_dimensions(max_keys);
        ir.rows = Some(rows as u8);
        ir.cols = Some(cols as u8);

        ir.warnings.push(format!(
            "Matrix dimensions ({}x{}) are guessed from {} keys. \
             Adjust rows/cols in keyboard.toml to match your actual hardware matrix.",
            rows, cols, max_keys
        ));

        // Reshape flat key list into rows×cols
        let mut reshaped_keymap: Vec<Vec<Vec<String>>> = Vec::new();
        for layer_keys in &keymap {
            let flat = &layer_keys[0];
            let mut layer_rows: Vec<Vec<String>> = Vec::new();
            for r in 0..rows {
                let mut row: Vec<String> = Vec::new();
                for c in 0..cols {
                    let idx = r * cols + c;
                    if idx < flat.len() {
                        row.push(flat[idx].clone());
                    } else {
                        row.push("_".into());
                    }
                }
                layer_rows.push(row);
            }
            reshaped_keymap.push(layer_rows);
        }
        ir.keymap = Some(reshaped_keymap);
    }

    Ok(ir)
}

/// Parse a ZMK .conf file and merge settings into IR.
pub(crate) fn parse_zmk_conf(path: &str, ir: &mut KeyboardIR) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_zmk_conf_str(&content, ir);
    Ok(())
}

/// Parse ZMK .conf content string and merge settings into IR.
pub(crate) fn parse_zmk_conf_str(content: &str, ir: &mut KeyboardIR) {
    for line in content.lines() {
        let line = line.trim();
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                "CONFIG_ZMK_KEYBOARD_NAME" => {
                    ir.name = Some(value.to_string());
                }
                "CONFIG_ZMK_SPLIT" => {
                    ir.is_split = Some(value == "y");
                }
                "CONFIG_ZMK_BLE" => {
                    if value == "y" {
                        ir.ble_enabled = Some(true);
                    }
                }
                _ => {}
            }
        }
    }
}

/// Extract layers from a .keymap file.
///
/// Returns Vec of (layer_name, bindings_raw_string).
/// Scans for `bindings = <` ... `>;` blocks within layer nodes.
fn extract_layers(content: &str) -> Vec<(String, String)> {
    let mut layers = Vec::new();
    let mut current_layer_name = String::new();
    let mut in_bindings = false;
    let mut bindings_buf = String::new();
    let mut brace_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track brace depth to identify layer nodes
        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                _ => {}
            }
        }

        // Detect layer node names: `something_layer {` or `default_layer {`
        // These appear at brace_depth 3 (root / { keymap { layer_name { )
        if !in_bindings && trimmed.ends_with('{') && brace_depth >= 3 {
            let name = trimmed.trim_end_matches('{').trim();
            if !name.is_empty()
                && name != "keymap"
                && name != "/"
                && !name.starts_with("compatible")
            {
                current_layer_name = name.to_string();
            }
        }

        // Check for `display-name = "..."` to get a nicer layer name
        if trimmed.starts_with("display-name") {
            if let Some((_k, v)) = trimmed.split_once('=') {
                let v = v.trim().trim_matches(';').trim().trim_matches('"');
                if !v.is_empty() {
                    current_layer_name = v.to_string();
                }
            }
        }

        // Start of bindings block
        if trimmed.contains("bindings = <") {
            in_bindings = true;
            // Capture everything after `bindings = <`
            if let Some(pos) = trimmed.find('<') {
                bindings_buf.push_str(&trimmed[pos + 1..]);
                bindings_buf.push(' ');
            }
            // Check if it also ends on the same line
            if trimmed.contains(">;") {
                if let Some(pos) = bindings_buf.rfind('>') {
                    bindings_buf.truncate(pos);
                }
                let name = if current_layer_name.is_empty() {
                    format!("layer_{}", layers.len())
                } else {
                    current_layer_name.clone()
                };
                layers.push((name, bindings_buf.trim().to_string()));
                bindings_buf = String::new();
                in_bindings = false;
            }
            continue;
        }

        if in_bindings {
            if trimmed.contains(">;") {
                // End of bindings
                let end_part = if let Some(pos) = trimmed.find('>') {
                    &trimmed[..pos]
                } else {
                    trimmed
                };
                bindings_buf.push_str(end_part);
                let name = if current_layer_name.is_empty() {
                    format!("layer_{}", layers.len())
                } else {
                    current_layer_name.clone()
                };
                layers.push((name, bindings_buf.trim().to_string()));
                bindings_buf = String::new();
                in_bindings = false;
            } else {
                bindings_buf.push_str(trimmed);
                bindings_buf.push(' ');
            }
        }
    }

    layers
}

/// Parse a raw bindings string into individual behavior bindings.
///
/// Input: "&kp TAB &kp Q &kp W &mo 1 &kp SPACE"
/// Output: ["&kp TAB", "&kp Q", "&kp W", "&mo 1", "&kp SPACE"]
fn parse_bindings(raw: &str) -> Vec<String> {
    let mut bindings = Vec::new();
    let mut current = String::new();

    for token in raw.split_whitespace() {
        if token.starts_with('&') {
            // Start of a new behavior
            if !current.is_empty() {
                bindings.push(current.trim().to_string());
            }
            current = token.to_string();
        } else {
            // Argument to current behavior
            current.push(' ');
            current.push_str(token);
        }
    }
    if !current.is_empty() {
        bindings.push(current.trim().to_string());
    }

    bindings
}

/// Guess a reasonable rows×cols factorization for a given key count.
fn guess_matrix_dimensions(key_count: usize) -> (usize, usize) {
    // Common keyboard sizes
    match key_count {
        42 => (7, 6),   // Corne 3×6+3 per side
        36 => (6, 6),   // Corne 3×5+3 per side
        48 => (8, 6),   // Lily58
        58 => (10, 6),  // Full 60%
        _ => {
            // Find a factorization where cols >= rows
            let sqrt = (key_count as f64).sqrt() as usize;
            for cols in (sqrt..=key_count).rev() {
                if key_count % cols == 0 {
                    let rows = key_count / cols;
                    return (rows, cols);
                }
            }
            (1, key_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bindings_basic() {
        let raw = "&kp TAB &kp Q &kp W &mo 1 &kp SPACE";
        let result = parse_bindings(raw);
        assert_eq!(result, vec![
            "&kp TAB", "&kp Q", "&kp W", "&mo 1", "&kp SPACE"
        ]);
    }

    #[test]
    fn test_parse_bindings_multi_arg() {
        let raw = "&kp A &lt 1 SPACE &bt BT_SEL 0 &trans";
        let result = parse_bindings(raw);
        assert_eq!(result, vec![
            "&kp A", "&lt 1 SPACE", "&bt BT_SEL 0", "&trans"
        ]);
    }

    #[test]
    fn test_extract_layers_corne() {
        let content = include_str!("../../tests/fixtures/zmk_corne/corne.keymap");
        let layers = extract_layers(content);

        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0].0, "Default Layer");
        assert_eq!(layers[1].0, "Lower Layer");
        assert_eq!(layers[2].0, "Raise Layer");

        // Default layer should have 42 bindings (Corne 3×6+3 per side)
        let bindings = parse_bindings(&layers[0].1);
        assert_eq!(bindings.len(), 42);
        assert_eq!(bindings[0], "&kp TAB");
        assert_eq!(bindings[1], "&kp Q");
    }

    #[test]
    fn test_parse_zmk_keymap_corne() {
        let content = include_str!("../../tests/fixtures/zmk_corne/corne.keymap");
        let ir = parse_zmk_keymap_str(content).unwrap();

        assert_eq!(ir.layers, Some(3));
        assert_eq!(ir.rows, Some(7)); // 42 keys → 7×6
        assert_eq!(ir.cols, Some(6));

        let keymap = ir.keymap.as_ref().unwrap();
        assert_eq!(keymap.len(), 3);
        // First key of first layer should be Tab
        assert_eq!(keymap[0][0][0], "Tab");
        // Second key should be Q
        assert_eq!(keymap[0][0][1], "Q");

        // Layer 1 should have numbers
        assert_eq!(keymap[1][0][1], "Kc1"); // N1 → Kc1

        // &trans should map to _
        assert!(keymap[1].iter().flatten().any(|k| k == "_"));

        // Should have warnings for &bt behaviors and shifted symbols
        assert!(!ir.warnings.is_empty());
        assert!(ir.warnings.iter().any(|w| w.contains("BT")));
        // Shifted symbols (EXCL, AT, etc.) in raise layer should produce warnings
        assert!(ir.warnings.iter().any(|w| w.contains("shifted symbol")));
        // Matrix guess warning
        assert!(ir.warnings.iter().any(|w| w.contains("guessed")));
    }

    #[test]
    fn test_parse_zmk_conf() {
        let mut ir = KeyboardIR::default();
        let conf = r#"
CONFIG_ZMK_KEYBOARD_NAME="My Corne"
CONFIG_ZMK_SPLIT=y
CONFIG_ZMK_BLE=y
# CONFIG_ZMK_RGB_UNDERGLOW=y
"#;
        parse_zmk_conf_str(conf, &mut ir);

        assert_eq!(ir.name.as_deref(), Some("My Corne"));
        assert_eq!(ir.is_split, Some(true));
        assert_eq!(ir.ble_enabled, Some(true));
    }

    #[test]
    fn test_parse_zmk_conf_comments_only() {
        let mut ir = KeyboardIR::default();
        let conf = include_str!("../../tests/fixtures/zmk_corne/corne.conf");
        parse_zmk_conf_str(conf, &mut ir);

        // The corne.conf has only comments, so nothing should be set
        assert_eq!(ir.name, None);
        assert_eq!(ir.is_split, None);
    }

    #[test]
    fn test_guess_matrix_dimensions() {
        assert_eq!(guess_matrix_dimensions(42), (7, 6));
        assert_eq!(guess_matrix_dimensions(36), (6, 6));
        assert_eq!(guess_matrix_dimensions(48), (8, 6));
        // Generic: 12 → 3×4
        let (r, c) = guess_matrix_dimensions(12);
        assert_eq!(r * c, 12);
        assert!(c >= r);
    }

    /// Integration: full ZMK pipeline — .keymap → keyboard.toml + vial.json
    #[test]
    fn test_full_zmk_pipeline_corne() {
        use crate::migrate::converter;

        let content = include_str!("../../tests/fixtures/zmk_corne/corne.keymap");
        let mut ir = parse_zmk_keymap_str(content).unwrap();
        ir.name = Some("Corne".into());
        ir.is_split = Some(true);
        ir.ble_enabled = Some(true);

        let toml = converter::generate_keyboard_toml(&ir, "nrf52840").unwrap();
        assert!(toml.contains("name = \"Corne\""));
        assert!(toml.contains("chip = \"nrf52840\""));
        assert!(toml.contains("rows = 7"));
        assert!(toml.contains("cols = 6"));
        assert!(toml.contains("layers = 3"));
        assert!(toml.contains("[ble]"));
        assert!(toml.contains("# TODO: Configure split keyboard settings"));
        // Verify keymap content
        assert!(toml.contains("\"Tab\""));
        assert!(toml.contains("\"Q\""));

        let vial = converter::generate_vial_json(&ir).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&vial).unwrap();
        assert_eq!(parsed["name"], "Corne");
        assert_eq!(parsed["matrix"]["rows"], 7);
        assert_eq!(parsed["matrix"]["cols"], 6);
    }
}
