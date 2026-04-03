use super::ir::KeyboardIR;
use serde_json::Value;
use std::error::Error;
use std::fs;

/// Parse a vial.json file and extract available keyboard information into IR.
pub(crate) fn parse_vial_json(path: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_vial_json_str(&content)
}

/// Parse vial.json content string into IR (testable without filesystem).
pub(crate) fn parse_vial_json_str(content: &str) -> Result<KeyboardIR, Box<dyn Error>> {
    let v: Value = serde_json::from_str(content)?;
    let mut ir = KeyboardIR::default();

    // Name
    if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
        ir.name = Some(name.to_string());
    }

    // Vendor ID — "0x4C4B" → 0x4C4B
    if let Some(vid_str) = v.get("vendorId").and_then(|n| n.as_str()) {
        if let Ok(vid) = parse_hex_id(vid_str) {
            ir.vendor_id = Some(vid);
        }
    }

    // Product ID
    if let Some(pid_str) = v.get("productId").and_then(|n| n.as_str()) {
        if let Ok(pid) = parse_hex_id(pid_str) {
            ir.product_id = Some(pid);
        }
    }

    // Matrix dimensions
    if let Some(matrix) = v.get("matrix") {
        if let Some(rows) = matrix.get("rows").and_then(|r| r.as_u64()) {
            ir.rows = Some(rows as u8);
        }
        if let Some(cols) = matrix.get("cols").and_then(|c| c.as_u64()) {
            ir.cols = Some(cols as u8);
        }
    }

    // Physical layout (KLE format) — preserve as-is for vial.json generation
    if let Some(layouts) = v.get("layouts") {
        if let Some(keymap) = layouts.get("keymap") {
            ir.physical_layout = Some(keymap.clone());
        }
    }

    // Custom keycodes — preserve as-is
    if let Some(custom) = v.get("customKeycodes") {
        ir.custom_keycodes = Some(custom.clone());
    }

    // Store the full vial.json so we can reuse it directly
    ir.vial_json_raw = Some(v);

    Ok(ir)
}

fn parse_hex_id(s: &str) -> Result<u16, Box<dyn Error>> {
    let s = s.trim();
    let s = if let Some(stripped) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        stripped
    } else {
        s
    };
    Ok(u16::from_str_radix(s, 16)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vial_json_basic() {
        let content = r#"{
            "name": "HID Keyboard",
            "vendorId": "0x4C4B",
            "productId": "0x4643",
            "matrix": { "rows": 4, "cols": 3 },
            "customKeycodes": [
                { "name": "BT0", "title": "Bluetooth Channel 0", "shortName": "BT0" }
            ],
            "layouts": {
                "keymap": [
                    ["0,0", "0,1", "0,2"],
                    ["1,0", "1,1", "1,2"]
                ]
            }
        }"#;

        let ir = parse_vial_json_str(content).unwrap();
        assert_eq!(ir.name.as_deref(), Some("HID Keyboard"));
        assert_eq!(ir.vendor_id, Some(0x4C4B));
        assert_eq!(ir.product_id, Some(0x4643));
        assert_eq!(ir.rows, Some(4));
        assert_eq!(ir.cols, Some(3));
        assert!(ir.physical_layout.is_some());
        assert!(ir.custom_keycodes.is_some());
        assert!(ir.vial_json_raw.is_some());
    }

    #[test]
    fn test_parse_vial_json_missing_optional_fields() {
        let content = r#"{
            "name": "Minimal",
            "matrix": { "rows": 2, "cols": 2 },
            "layouts": { "keymap": [] }
        }"#;

        let ir = parse_vial_json_str(content).unwrap();
        assert_eq!(ir.name.as_deref(), Some("Minimal"));
        assert_eq!(ir.vendor_id, None);
        assert_eq!(ir.product_id, None);
        assert_eq!(ir.rows, Some(2));
        assert_eq!(ir.cols, Some(2));
        assert!(ir.custom_keycodes.is_none());
    }

    /// Integration test: parse the repo's actual vial.json
    #[test]
    fn test_parse_repo_vial_json() {
        let repo_vial = include_str!("../../vial.json");
        let ir = parse_vial_json_str(repo_vial).unwrap();

        assert_eq!(ir.name.as_deref(), Some("HID Keyboard"));
        assert_eq!(ir.vendor_id, Some(0x4C4B));
        assert_eq!(ir.product_id, Some(0x4643));
        assert_eq!(ir.rows, Some(4));
        assert_eq!(ir.cols, Some(3));
        assert!(ir.physical_layout.is_some());
        assert!(ir.custom_keycodes.is_some());

        // Verify custom keycodes contain BT entries
        let custom = ir.custom_keycodes.unwrap();
        let arr = custom.as_array().unwrap();
        assert_eq!(arr.len(), 12);
        assert_eq!(arr[0]["name"], "BT0");
    }

    #[test]
    fn test_parse_hex_id() {
        assert_eq!(parse_hex_id("0x4C4B").unwrap(), 0x4C4B);
        assert_eq!(parse_hex_id("0X00FF").unwrap(), 0x00FF);
        assert_eq!(parse_hex_id("FEED").unwrap(), 0xFEED);
    }
}
