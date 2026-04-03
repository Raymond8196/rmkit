use super::ir::KeyboardIR;
use super::keycode_map;
use serde_json::Value;
use std::error::Error;
use std::fs;

/// Parse QMK info.json (or keyboard.json) into IR.
pub(crate) fn parse_qmk_info_json(path: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_qmk_info_json_str(&content)
}

/// Parse QMK info.json content string into IR.
pub(crate) fn parse_qmk_info_json_str(content: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let v: Value = serde_json::from_str(content)?;
    let mut ir = KeyboardIR::default();

    // Keyboard name
    if let Some(name) = v.get("keyboard_name").and_then(|n| n.as_str()) {
        ir.name = Some(name.to_string());
    }

    // Manufacturer
    if let Some(mfr) = v.get("manufacturer").and_then(|n| n.as_str()) {
        ir.manufacturer = Some(mfr.to_string());
    }

    // USB VID/PID
    if let Some(usb) = v.get("usb") {
        if let Some(vid_str) = usb.get("vid").and_then(|n| n.as_str()) {
            ir.vendor_id = parse_hex(vid_str);
        }
        if let Some(pid_str) = usb.get("pid").and_then(|n| n.as_str()) {
            ir.product_id = parse_hex(pid_str);
        }
    }

    // Processor → chip hint
    if let Some(proc_str) = v.get("processor").and_then(|n| n.as_str()) {
        ir.chip_hint = keycode_map::map_qmk_processor(proc_str);
        if ir.chip_hint.is_none() {
            ir.warnings.push(format!(
                "QMK processor '{}' has no RMK equivalent. You'll need to select a target chip.",
                proc_str
            ));
        }
    }

    // Matrix dimensions from matrix_pins
    if let Some(matrix_pins) = v.get("matrix_pins") {
        if let Some(direct) = matrix_pins.get("direct").and_then(|d| d.as_array()) {
            // Direct pin matrix: rows = array length, cols = max inner array length
            ir.rows = Some(direct.len() as u8);
            let max_cols = direct
                .iter()
                .filter_map(|row| row.as_array().map(|a| a.len()))
                .max()
                .unwrap_or(0);
            ir.cols = Some(max_cols as u8);
        } else {
            if let Some(rows) = matrix_pins.get("rows").and_then(|r| r.as_array()) {
                ir.rows = Some(rows.len() as u8);
            }
            if let Some(cols) = matrix_pins.get("cols").and_then(|c| c.as_array()) {
                ir.cols = Some(cols.len() as u8);
            }
        }
    }

    // Diode direction
    if let Some(dir) = v.get("diode_direction").and_then(|d| d.as_str()) {
        ir.diode_direction = Some(dir.to_lowercase());
    }

    // Split detection
    if let Some(split) = v.get("split") {
        if let Some(enabled) = split.get("enabled").and_then(|e| e.as_bool()) {
            ir.is_split = Some(enabled);
        }
    }

    // Physical layout — convert to KLE format.
    // Prefer "LAYOUT" key, otherwise pick the first sorted layout name for determinism.
    if let Some(layouts) = v.get("layouts").and_then(|l| l.as_object()) {
        let layout_obj = layouts.get("LAYOUT").or_else(|| {
            let mut keys: Vec<&String> = layouts.keys().collect();
            keys.sort();
            keys.first().and_then(|k| layouts.get(k.as_str()))
        });
        if let Some(layout_obj) = layout_obj {
            if let Some(layout_arr) = layout_obj.get("layout").and_then(|l| l.as_array()) {
                ir.physical_layout = Some(qmk_layout_to_kle(layout_arr));
            }
        }
    }

    Ok(ir)
}

/// Parse QMK keymap.json and merge keycodes into IR.
pub(crate) fn parse_qmk_keymap_json(path: &str, ir: &mut KeyboardIR) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_qmk_keymap_json_str(&content, ir)
}

/// Parse QMK keymap.json content string and merge keycodes into IR.
pub(crate) fn parse_qmk_keymap_json_str(
    content: &str,
    ir: &mut KeyboardIR,
) -> Result<(), Box<dyn Error>> {
    let v: Value = serde_json::from_str(content)?;

    if let Some(layers) = v.get("layers").and_then(|l| l.as_array()) {
        let rows = ir.rows.unwrap_or(1) as usize;
        let cols = ir.cols.unwrap_or(1) as usize;
        ir.layers = Some(layers.len() as u8);

        let mut keymap = Vec::new();
        for (layer_idx, layer) in layers.iter().enumerate() {
            if let Some(keys) = layer.as_array() {
                // QMK keymap.json stores keys as a flat array per layer.
                // We need to reshape into rows × cols.
                let mut layer_map: Vec<Vec<String>> = Vec::new();
                for r in 0..rows {
                    let mut row_map = Vec::new();
                    for c in 0..cols {
                        let idx = r * cols + c;
                        let rmk_key = if let Some(qmk_key) = keys.get(idx).and_then(|k| k.as_str())
                        {
                            match keycode_map::map_qmk_keycode(qmk_key) {
                                Ok(mapped) => mapped,
                                Err(warning) => {
                                    ir.warnings.push(format!(
                                        "Layer {}, position [{},{}]: {}",
                                        layer_idx, r, c, warning
                                    ));
                                    "_".to_string()
                                }
                            }
                        } else {
                            "_".to_string()
                        };
                        row_map.push(rmk_key);
                    }
                    layer_map.push(row_map);
                }
                keymap.push(layer_map);
            }
        }
        ir.keymap = Some(keymap);
    }

    Ok(())
}

/// Convert QMK layout array to Vial KLE format.
///
/// QMK format: [{"matrix": [0,0], "x": 0, "y": 0}, ...]
/// KLE format:  [["0,0", "0,1", ...], [...]] grouped by y-rows
fn qmk_layout_to_kle(layout: &[Value]) -> Value {
    // Collect all keys with their positions
    let mut keys: Vec<(f64, f64, String, Option<f64>, Option<f64>)> = Vec::new();

    for key in layout {
        let x = key.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let y = key.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let w = key.get("w").and_then(|v| v.as_f64());
        let h = key.get("h").and_then(|v| v.as_f64());

        let matrix_str = if let Some(matrix) = key.get("matrix").and_then(|m| m.as_array()) {
            if matrix.len() == 2 {
                format!(
                    "{},{}",
                    matrix[0].as_u64().unwrap_or(0),
                    matrix[1].as_u64().unwrap_or(0)
                )
            } else {
                "0,0".into()
            }
        } else {
            "0,0".into()
        };

        keys.push((x, y, matrix_str, w, h));
    }

    // Sort by y then x
    keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().then(a.0.partial_cmp(&b.0).unwrap()));

    // Group into KLE rows by tracking position
    let mut kle_rows: Vec<Value> = Vec::new();
    let mut current_row: Vec<Value> = Vec::new();
    let mut cursor_x: f64 = 0.0;
    let mut cursor_y: f64 = 0.0;

    for (x, y, matrix_str, w, h) in &keys {
        // Check if we need a new row (significant y change)
        if !current_row.is_empty() && (*y - cursor_y).abs() > 0.01 {
            kle_rows.push(Value::Array(current_row));
            current_row = Vec::new();
            cursor_x = 0.0;
        }

        // Add position metadata if needed
        let dx = *x - cursor_x;
        let dy = *y - cursor_y;
        let has_offset = dx.abs() > 0.01 || (current_row.is_empty() && dy.abs() > 0.01);
        let has_size = w.is_some() || h.is_some();

        if has_offset || has_size {
            let mut meta = serde_json::Map::new();
            if current_row.is_empty() && dy.abs() > 0.01 {
                meta.insert("y".into(), Value::Number(serde_json::Number::from_f64(dy).unwrap()));
            }
            if dx.abs() > 0.01 {
                meta.insert("x".into(), Value::Number(serde_json::Number::from_f64(dx).unwrap()));
            }
            if let Some(w_val) = w {
                if (*w_val - 1.0).abs() > 0.01 {
                    meta.insert("w".into(), Value::Number(serde_json::Number::from_f64(*w_val).unwrap()));
                }
            }
            if let Some(h_val) = h {
                if (*h_val - 1.0).abs() > 0.01 {
                    meta.insert("h".into(), Value::Number(serde_json::Number::from_f64(*h_val).unwrap()));
                }
            }
            if !meta.is_empty() {
                current_row.push(Value::Object(meta));
            }
        }

        current_row.push(Value::String(matrix_str.clone()));
        cursor_x = *x + w.unwrap_or(1.0);
        cursor_y = *y;
    }

    if !current_row.is_empty() {
        kle_rows.push(Value::Array(current_row));
    }

    Value::Array(kle_rows)
}

fn parse_hex(s: &str) -> Option<u16> {
    let s = s.trim();
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u16::from_str_radix(s, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qmk_macropad() {
        let content = include_str!("../../tests/fixtures/qmk_macropad/keyboard.json");
        let ir = parse_qmk_info_json_str(content).unwrap();

        assert_eq!(ir.name.as_deref(), Some("Macropad RP2040"));
        assert_eq!(ir.manufacturer.as_deref(), Some("Adafruit"));
        assert_eq!(ir.vendor_id, Some(0x239A));
        assert_eq!(ir.product_id, Some(0x0108));
        assert_eq!(ir.chip_hint.as_deref(), Some("rp2040"));
        // Direct pin matrix: 5 rows, 3 cols
        assert_eq!(ir.rows, Some(5));
        assert_eq!(ir.cols, Some(3));
        assert_eq!(ir.is_split, None);
        assert!(ir.physical_layout.is_some());
        assert!(ir.warnings.is_empty());
    }

    #[test]
    fn test_parse_qmk_crkbd() {
        let content = include_str!("../../tests/fixtures/qmk_crkbd/info.json");
        let ir = parse_qmk_info_json_str(content).unwrap();

        assert_eq!(ir.manufacturer.as_deref(), Some("foostan"));
        assert_eq!(ir.vendor_id, Some(0x4653));
        assert_eq!(ir.is_split, Some(true));
        // crkbd info.json has no keyboard_name
        assert_eq!(ir.name, None);
        // crkbd info.json has no processor or matrix_pins at this level
        assert!(ir.physical_layout.is_some());
    }

    #[test]
    fn test_parse_qmk_keymap_json() {
        let info = r#"{
            "matrix_pins": { "rows": ["R0", "R1"], "cols": ["C0", "C1", "C2"] }
        }"#;
        let mut ir = parse_qmk_info_json_str(info).unwrap();
        assert_eq!(ir.rows, Some(2));
        assert_eq!(ir.cols, Some(3));

        let keymap = r#"{
            "keyboard": "test",
            "keymap": "default",
            "layout": "LAYOUT",
            "layers": [
                ["KC_A", "KC_B", "KC_C", "KC_1", "KC_2", "KC_3"],
                ["KC_TRNS", "MO(1)", "KC_TRNS", "KC_TRNS", "KC_TRNS", "KC_TRNS"]
            ]
        }"#;
        parse_qmk_keymap_json_str(keymap, &mut ir).unwrap();

        assert_eq!(ir.layers, Some(2));
        let km = ir.keymap.unwrap();
        assert_eq!(km.len(), 2);
        assert_eq!(km[0][0], vec!["A", "B", "C"]);
        assert_eq!(km[0][1], vec!["Kc1", "Kc2", "Kc3"]);
        assert_eq!(km[1][0][1], "MO(1)");
    }

    #[test]
    fn test_parse_qmk_keymap_with_warnings() {
        let mut ir = KeyboardIR {
            rows: Some(1),
            cols: Some(3),
            ..Default::default()
        };

        let keymap = r#"{
            "layers": [
                ["KC_A", "QK_BOOT", "RGB_TOG"]
            ]
        }"#;
        parse_qmk_keymap_json_str(keymap, &mut ir).unwrap();

        let km = ir.keymap.unwrap();
        assert_eq!(km[0][0][0], "A");
        assert_eq!(km[0][0][1], "_"); // unmapped → transparent
        assert_eq!(km[0][0][2], "_"); // unmapped → transparent
        assert_eq!(ir.warnings.len(), 2);
        assert!(ir.warnings[0].contains("QK_BOOT"));
        assert!(ir.warnings[1].contains("RGB_TOG"));
    }

    #[test]
    fn test_qmk_layout_to_kle_basic() {
        let layout: Vec<Value> = serde_json::from_str(
            r#"[
                {"matrix": [0, 0], "x": 0, "y": 0},
                {"matrix": [0, 1], "x": 1, "y": 0},
                {"matrix": [1, 0], "x": 0, "y": 1},
                {"matrix": [1, 1], "x": 1, "y": 1}
            ]"#,
        )
        .unwrap();

        let kle = qmk_layout_to_kle(&layout);
        let rows = kle.as_array().unwrap();
        assert_eq!(rows.len(), 2); // 2 KLE rows

        // First row should have "0,0", "0,1"
        let row0 = rows[0].as_array().unwrap();
        assert!(row0.iter().any(|v| v.as_str() == Some("0,0")));
        assert!(row0.iter().any(|v| v.as_str() == Some("0,1")));
    }

    #[test]
    fn test_qmk_layout_to_kle_macropad() {
        let content = include_str!("../../tests/fixtures/qmk_macropad/keyboard.json");
        let v: Value = serde_json::from_str(content).unwrap();
        let layout = v["layouts"]["LAYOUT"]["layout"].as_array().unwrap();
        let kle = qmk_layout_to_kle(layout);

        // MacroPad has 13 keys across 5 y-positions
        let rows = kle.as_array().unwrap();
        assert!(rows.len() >= 4); // at least 4 visual rows
    }

    /// Integration: full QMK pipeline — info.json + keymap.json → keyboard.toml + vial.json
    #[test]
    fn test_full_qmk_pipeline_macropad() {
        use crate::migrate::converter;

        let info_content = include_str!("../../tests/fixtures/qmk_macropad/keyboard.json");
        let mut ir = parse_qmk_info_json_str(info_content).unwrap();

        // Simulate a keymap.json for the macropad (5 rows × 3 cols = 15 keys, 13 used + 2 null)
        let keymap_json = r#"{
            "keyboard": "adafruit/macropad",
            "keymap": "test",
            "layout": "LAYOUT",
            "layers": [
                ["KC_MUTE", "KC_NO", "KC_NO",
                 "KC_1", "KC_2", "KC_3",
                 "KC_4", "KC_5", "KC_6",
                 "KC_7", "KC_8", "KC_9",
                 "KC_0", "KC_ENT", "KC_BSPC"],
                ["KC_TRNS", "KC_TRNS", "KC_TRNS",
                 "KC_F1", "KC_F2", "KC_F3",
                 "KC_F4", "KC_F5", "KC_F6",
                 "KC_F7", "KC_F8", "KC_F9",
                 "KC_F10", "KC_F11", "KC_F12"]
            ]
        }"#;
        parse_qmk_keymap_json_str(keymap_json, &mut ir).unwrap();

        // Set remaining fields
        ir.layers = Some(2);
        ir.ble_enabled = Some(false);

        // Generate keyboard.toml
        let toml = converter::generate_keyboard_toml(&ir, "rp2040").unwrap();
        assert!(toml.contains("name = \"Macropad_RP2040\""));
        assert!(toml.contains("vendor_id = 0x239A"));
        assert!(toml.contains("product_id = 0x0108"));
        assert!(toml.contains("chip = \"rp2040\""));
        assert!(toml.contains("rows = 5"));
        assert!(toml.contains("cols = 3"));
        // Verify keymap got mapped correctly
        assert!(toml.contains("\"AudioMute\"")); // KC_MUTE → AudioMute
        assert!(toml.contains("\"Kc1\"")); // KC_1 → Kc1
        assert!(toml.contains("\"F1\"")); // KC_F1 → F1

        // Generate vial.json from layout (no raw vial.json provided)
        let vial = converter::generate_vial_json(&ir).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&vial).unwrap();
        assert_eq!(parsed["name"], "Macropad RP2040");
        assert_eq!(parsed["matrix"]["rows"], 5);
        assert_eq!(parsed["matrix"]["cols"], 3);
        // Should have generated a KLE layout from QMK layout
        assert!(parsed["layouts"]["keymap"].is_array());

        assert!(ir.warnings.is_empty());
    }

    #[test]
    fn test_processor_mapping_in_parse() {
        let content = r#"{
            "processor": "atmega32u4"
        }"#;
        let ir = parse_qmk_info_json_str(content).unwrap();
        assert_eq!(ir.chip_hint, None);
        assert_eq!(ir.warnings.len(), 1);
        assert!(ir.warnings[0].contains("atmega32u4"));
    }
}
