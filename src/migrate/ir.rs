/// Intermediate representation of a keyboard configuration.
///
/// All fields are `Option` — parsers populate what they can extract,
/// and the migrate flow interactively prompts for anything still missing.
#[derive(Debug, Default)]
pub(crate) struct KeyboardIR {
    // Basic info
    pub name: Option<String>,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub manufacturer: Option<String>,

    // Matrix
    pub rows: Option<u8>,
    pub cols: Option<u8>,
    pub layers: Option<u8>,

    // Keymap: [layer][row][col] = RMK keycode string
    pub keymap: Option<Vec<Vec<Vec<String>>>>,

    // Hardware hints
    pub chip_hint: Option<String>,
    pub input_pins: Option<Vec<String>>,
    pub output_pins: Option<Vec<String>>,
    #[allow(dead_code)]
    pub diode_direction: Option<String>,

    // Split
    pub is_split: Option<bool>,

    // BLE
    pub ble_enabled: Option<bool>,

    // Layout-to-matrix mapping: layout_matrix_map[i] = (row, col) for the i-th key in layout order.
    // Used to correctly place flat keymap arrays into the 2D matrix.
    pub layout_matrix_map: Option<Vec<(usize, usize)>>,

    // Physical layout — raw KLE JSON for vial.json
    pub physical_layout: Option<serde_json::Value>,

    // Custom keycodes from Vial
    pub custom_keycodes: Option<serde_json::Value>,

    // Full vial.json content (when source already provides it)
    pub vial_json_raw: Option<serde_json::Value>,

    // Warnings for unmapped keycodes or unsupported features
    pub warnings: Vec<String>,
}
