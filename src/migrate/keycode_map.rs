use std::collections::HashMap;
use std::sync::LazyLock;

/// Static mapping from QMK keycode names to RMK keycode names.
///
/// Covers ~120 most commonly used QMK keycodes.
/// QMK keys are stored in UPPERCASE for case-insensitive lookup.
static QMK_TO_RMK: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // Letters
    for c in b'A'..=b'Z' {
        let s: &'static str = match c {
            b'A' => "A", b'B' => "B", b'C' => "C", b'D' => "D", b'E' => "E",
            b'F' => "F", b'G' => "G", b'H' => "H", b'I' => "I", b'J' => "J",
            b'K' => "K", b'L' => "L", b'M' => "M", b'N' => "N", b'O' => "O",
            b'P' => "P", b'Q' => "Q", b'R' => "R", b'S' => "S", b'T' => "T",
            b'U' => "U", b'V' => "V", b'W' => "W", b'X' => "X", b'Y' => "Y",
            b'Z' => "Z", _ => unreachable!(),
        };
        // QMK uses KC_A, KC_B, etc.
        let qmk: &'static str = match c {
            b'A' => "KC_A", b'B' => "KC_B", b'C' => "KC_C", b'D' => "KC_D", b'E' => "KC_E",
            b'F' => "KC_F", b'G' => "KC_G", b'H' => "KC_H", b'I' => "KC_I", b'J' => "KC_J",
            b'K' => "KC_K", b'L' => "KC_L", b'M' => "KC_M", b'N' => "KC_N", b'O' => "KC_O",
            b'P' => "KC_P", b'Q' => "KC_Q", b'R' => "KC_R", b'S' => "KC_S", b'T' => "KC_T",
            b'U' => "KC_U", b'V' => "KC_V", b'W' => "KC_W", b'X' => "KC_X", b'Y' => "KC_Y",
            b'Z' => "KC_Z", _ => unreachable!(),
        };
        m.insert(qmk, s);
    }

    // Numbers (top row)
    m.insert("KC_1", "Kc1");
    m.insert("KC_2", "Kc2");
    m.insert("KC_3", "Kc3");
    m.insert("KC_4", "Kc4");
    m.insert("KC_5", "Kc5");
    m.insert("KC_6", "Kc6");
    m.insert("KC_7", "Kc7");
    m.insert("KC_8", "Kc8");
    m.insert("KC_9", "Kc9");
    m.insert("KC_0", "Kc0");

    // Special keys
    m.insert("KC_ENT", "Enter");
    m.insert("KC_ENTER", "Enter");
    m.insert("KC_ESC", "Escape");
    m.insert("KC_ESCAPE", "Escape");
    m.insert("KC_BSPC", "Backspace");
    m.insert("KC_BACKSPACE", "Backspace");
    m.insert("KC_TAB", "Tab");
    m.insert("KC_SPC", "Space");
    m.insert("KC_SPACE", "Space");
    m.insert("KC_MINS", "Minus");
    m.insert("KC_MINUS", "Minus");
    m.insert("KC_EQL", "Equal");
    m.insert("KC_EQUAL", "Equal");
    m.insert("KC_LBRC", "LeftBracket");
    m.insert("KC_LEFT_BRACKET", "LeftBracket");
    m.insert("KC_RBRC", "RightBracket");
    m.insert("KC_RIGHT_BRACKET", "RightBracket");
    m.insert("KC_BSLS", "Backslash");
    m.insert("KC_BACKSLASH", "Backslash");
    m.insert("KC_NUHS", "NonusHash");
    m.insert("KC_NONUS_HASH", "NonusHash");
    m.insert("KC_SCLN", "Semicolon");
    m.insert("KC_SEMICOLON", "Semicolon");
    m.insert("KC_QUOT", "Quote");
    m.insert("KC_QUOTE", "Quote");
    m.insert("KC_GRV", "Grave");
    m.insert("KC_GRAVE", "Grave");
    m.insert("KC_COMM", "Comma");
    m.insert("KC_COMMA", "Comma");
    m.insert("KC_DOT", "Dot");
    m.insert("KC_SLSH", "Slash");
    m.insert("KC_SLASH", "Slash");
    m.insert("KC_CAPS", "CapsLock");
    m.insert("KC_CAPS_LOCK", "CapsLock");

    // Function keys
    m.insert("KC_F1", "F1");
    m.insert("KC_F2", "F2");
    m.insert("KC_F3", "F3");
    m.insert("KC_F4", "F4");
    m.insert("KC_F5", "F5");
    m.insert("KC_F6", "F6");
    m.insert("KC_F7", "F7");
    m.insert("KC_F8", "F8");
    m.insert("KC_F9", "F9");
    m.insert("KC_F10", "F10");
    m.insert("KC_F11", "F11");
    m.insert("KC_F12", "F12");
    m.insert("KC_F13", "F13");
    m.insert("KC_F14", "F14");
    m.insert("KC_F15", "F15");
    m.insert("KC_F16", "F16");
    m.insert("KC_F17", "F17");
    m.insert("KC_F18", "F18");
    m.insert("KC_F19", "F19");
    m.insert("KC_F20", "F20");
    m.insert("KC_F21", "F21");
    m.insert("KC_F22", "F22");
    m.insert("KC_F23", "F23");
    m.insert("KC_F24", "F24");

    // Navigation
    m.insert("KC_PSCR", "PrintScreen");
    m.insert("KC_PRINT_SCREEN", "PrintScreen");
    m.insert("KC_SCRL", "ScrollLock");
    m.insert("KC_SCROLL_LOCK", "ScrollLock");
    m.insert("KC_PAUS", "Pause");
    m.insert("KC_PAUSE", "Pause");
    m.insert("KC_INS", "Insert");
    m.insert("KC_INSERT", "Insert");
    m.insert("KC_HOME", "Home");
    m.insert("KC_PGUP", "PageUp");
    m.insert("KC_PAGE_UP", "PageUp");
    m.insert("KC_DEL", "Delete");
    m.insert("KC_DELETE", "Delete");
    m.insert("KC_END", "End");
    m.insert("KC_PGDN", "PageDown");
    m.insert("KC_PAGE_DOWN", "PageDown");
    m.insert("KC_RGHT", "Right");
    m.insert("KC_RIGHT", "Right");
    m.insert("KC_LEFT", "Left");
    m.insert("KC_DOWN", "Down");
    m.insert("KC_UP", "Up");

    // Modifiers
    m.insert("KC_LCTL", "LCtrl");
    m.insert("KC_LCTRL", "LCtrl");
    m.insert("KC_LEFT_CTRL", "LCtrl");
    m.insert("KC_LSFT", "LShift");
    m.insert("KC_LSHIFT", "LShift");
    m.insert("KC_LEFT_SHIFT", "LShift");
    m.insert("KC_LALT", "LAlt");
    m.insert("KC_LEFT_ALT", "LAlt");
    m.insert("KC_LOPT", "LAlt");
    m.insert("KC_LGUI", "LGui");
    m.insert("KC_LEFT_GUI", "LGui");
    m.insert("KC_LCMD", "LGui");
    m.insert("KC_LWIN", "LGui");
    m.insert("KC_RCTL", "RCtrl");
    m.insert("KC_RCTRL", "RCtrl");
    m.insert("KC_RIGHT_CTRL", "RCtrl");
    m.insert("KC_RSFT", "RShift");
    m.insert("KC_RSHIFT", "RShift");
    m.insert("KC_RIGHT_SHIFT", "RShift");
    m.insert("KC_RALT", "RAlt");
    m.insert("KC_RIGHT_ALT", "RAlt");
    m.insert("KC_ROPT", "RAlt");
    m.insert("KC_ALGR", "RAlt");
    m.insert("KC_RGUI", "RGui");
    m.insert("KC_RIGHT_GUI", "RGui");
    m.insert("KC_RCMD", "RGui");
    m.insert("KC_RWIN", "RGui");

    // Numpad
    m.insert("KC_NUM", "NumLock");
    m.insert("KC_NUM_LOCK", "NumLock");
    m.insert("KC_PSLS", "KpSlash");
    m.insert("KC_KP_SLASH", "KpSlash");
    m.insert("KC_PAST", "KpAsterisk");
    m.insert("KC_KP_ASTERISK", "KpAsterisk");
    m.insert("KC_PMNS", "KpMinus");
    m.insert("KC_KP_MINUS", "KpMinus");
    m.insert("KC_PPLS", "KpPlus");
    m.insert("KC_KP_PLUS", "KpPlus");
    m.insert("KC_PENT", "KpEnter");
    m.insert("KC_KP_ENTER", "KpEnter");
    m.insert("KC_P1", "Kp1");
    m.insert("KC_KP_1", "Kp1");
    m.insert("KC_P2", "Kp2");
    m.insert("KC_KP_2", "Kp2");
    m.insert("KC_P3", "Kp3");
    m.insert("KC_KP_3", "Kp3");
    m.insert("KC_P4", "Kp4");
    m.insert("KC_KP_4", "Kp4");
    m.insert("KC_P5", "Kp5");
    m.insert("KC_KP_5", "Kp5");
    m.insert("KC_P6", "Kp6");
    m.insert("KC_KP_6", "Kp6");
    m.insert("KC_P7", "Kp7");
    m.insert("KC_KP_7", "Kp7");
    m.insert("KC_P8", "Kp8");
    m.insert("KC_KP_8", "Kp8");
    m.insert("KC_P9", "Kp9");
    m.insert("KC_KP_9", "Kp9");
    m.insert("KC_P0", "Kp0");
    m.insert("KC_KP_0", "Kp0");
    m.insert("KC_PDOT", "KpDot");
    m.insert("KC_KP_DOT", "KpDot");
    m.insert("KC_PEQL", "KpEqual");
    m.insert("KC_KP_EQUAL", "KpEqual");

    // Misc
    m.insert("KC_NUBS", "NonusBackslash");
    m.insert("KC_NONUS_BACKSLASH", "NonusBackslash");
    m.insert("KC_APP", "Application");
    m.insert("KC_APPLICATION", "Application");
    m.insert("KC_EXEC", "Execute");
    m.insert("KC_EXECUTE", "Execute");
    m.insert("KC_HELP", "Help");
    m.insert("KC_MENU", "Menu");
    m.insert("KC_SLCT", "Select");
    m.insert("KC_SELECT", "Select");
    m.insert("KC_STOP", "Stop");
    m.insert("KC_AGIN", "Again");
    m.insert("KC_AGAIN", "Again");
    m.insert("KC_UNDO", "Undo");
    m.insert("KC_CUT", "Cut");
    m.insert("KC_COPY", "Copy");
    m.insert("KC_PSTE", "Paste");
    m.insert("KC_PASTE", "Paste");
    m.insert("KC_FIND", "Find");

    // Media / System
    m.insert("KC_MUTE", "AudioMute");
    m.insert("KC_AUDIO_MUTE", "AudioMute");
    m.insert("KC_VOLU", "AudioVolUp");
    m.insert("KC_AUDIO_VOL_UP", "AudioVolUp");
    m.insert("KC_VOLD", "AudioVolDown");
    m.insert("KC_AUDIO_VOL_DOWN", "AudioVolDown");
    m.insert("KC_MNXT", "MediaNextTrack");
    m.insert("KC_MEDIA_NEXT_TRACK", "MediaNextTrack");
    m.insert("KC_MPRV", "MediaPrevTrack");
    m.insert("KC_MEDIA_PREV_TRACK", "MediaPrevTrack");
    m.insert("KC_MSTP", "MediaStop");
    m.insert("KC_MEDIA_STOP", "MediaStop");
    m.insert("KC_MPLY", "MediaPlayPause");
    m.insert("KC_MEDIA_PLAY_PAUSE", "MediaPlayPause");
    m.insert("KC_MSEL", "MediaSelect");
    m.insert("KC_MEDIA_SELECT", "MediaSelect");
    m.insert("KC_EJCT", "MediaEject");
    m.insert("KC_MEDIA_EJECT", "MediaEject");
    m.insert("KC_MFFD", "MediaFastForward");
    m.insert("KC_MEDIA_FAST_FORWARD", "MediaFastForward");
    m.insert("KC_MRWD", "MediaRewind");
    m.insert("KC_MEDIA_REWIND", "MediaRewind");
    m.insert("KC_BRIU", "BrightnessUp");
    m.insert("KC_BRIGHTNESS_UP", "BrightnessUp");
    m.insert("KC_BRID", "BrightnessDown");
    m.insert("KC_BRIGHTNESS_DOWN", "BrightnessDown");
    m.insert("KC_PWR", "SystemPower");
    m.insert("KC_SYSTEM_POWER", "SystemPower");
    m.insert("KC_SLEP", "SystemSleep");
    m.insert("KC_SYSTEM_SLEEP", "SystemSleep");
    m.insert("KC_WAKE", "SystemWake");
    m.insert("KC_SYSTEM_WAKE", "SystemWake");

    // Transparent / No-op
    m.insert("KC_TRNS", "_");
    m.insert("KC_TRANSPARENT", "_");
    m.insert("_______", "_");
    m.insert("KC_NO", "No");
    m.insert("XXXXXXX", "No");

    m
});

/// Map a single QMK keycode string to an RMK keycode string.
///
/// Handles:
/// - Simple keycodes: "KC_A" → "A"
/// - Layer functions: "MO(1)" → "MO(1)", "LT(1, KC_SPC)" → "LT(1, Space)"
/// - One-shot: "OSL(1)" → "OSL(1)", "OSM(MOD_LSFT)" → "OSM(LShift)"
/// - Toggle: "TG(1)" → "TG(1)", "TT(1)" → "TT(1)"
///
/// Returns Err with the original keycode if unmappable.
pub(crate) fn map_qmk_keycode(qmk: &str) -> Result<String, String> {
    let trimmed = qmk.trim();
    let upper = trimmed.to_uppercase();

    // 1. Check simple lookup first
    if let Some(rmk) = QMK_TO_RMK.get(upper.as_str()) {
        return Ok(rmk.to_string());
    }

    // 2. Handle function-style keycodes
    if let Some((func, args)) = parse_func_keycode(trimmed) {
        let func_upper = func.to_uppercase();
        match func_upper.as_str() {
            // Layer operations that pass through directly with just a layer number
            "MO" | "TG" | "TT" | "OSL" | "DF" | "TO" => {
                let layer = args.trim();
                if layer.parse::<u8>().is_ok() {
                    return Ok(format!("{}({})", func_upper, layer));
                }
                return Err(format!("Invalid layer number in {}({})", func, args));
            }
            // LT(layer, keycode) — need to map the inner keycode
            "LT" => {
                let parts: Vec<&str> = args.splitn(2, ',').collect();
                if parts.len() == 2 {
                    let layer = parts[0].trim();
                    let inner_kc = parts[1].trim();
                    if layer.parse::<u8>().is_ok() {
                        match map_qmk_keycode(inner_kc) {
                            Ok(mapped) => return Ok(format!("LT({}, {})", layer, mapped)),
                            Err(_) => return Err(format!("Unmapped keycode in LT: {}", inner_kc)),
                        }
                    }
                }
                return Err(format!("Invalid LT args: {}", args));
            }
            // LM(layer, modifier) — map the modifier
            "LM" => {
                let parts: Vec<&str> = args.splitn(2, ',').collect();
                if parts.len() == 2 {
                    let layer = parts[0].trim();
                    let mod_str = parts[1].trim();
                    if layer.parse::<u8>().is_ok() {
                        let mapped_mods = map_qmk_modifier_combo(mod_str);
                        return Ok(format!("LM({}, {})", layer, mapped_mods));
                    }
                }
                return Err(format!("Invalid LM args: {}", args));
            }
            // OSM(modifier)
            "OSM" => {
                let mapped_mod = map_qmk_modifier(args.trim());
                return Ok(format!("OSM({})", mapped_mod));
            }
            _ => {}
        }
    }

    Err(format!("Unmapped QMK keycode: {}", trimmed))
}

/// Parse a function-style keycode like "MO(1)" or "LT(1, KC_SPC)"
/// Returns (function_name, inner_args) or None
fn parse_func_keycode(s: &str) -> Option<(&str, &str)> {
    let open = s.find('(')?;
    let close = s.rfind(')')?;
    if close > open {
        Some((&s[..open], &s[open + 1..close]))
    } else {
        None
    }
}

/// Map a QMK modifier constant to RMK modifier name
fn map_qmk_modifier(qmk_mod: &str) -> String {
    match qmk_mod.to_uppercase().as_str() {
        "MOD_LSFT" | "MOD_LSHIFT" => "LShift".into(),
        "MOD_RSFT" | "MOD_RSHIFT" => "RShift".into(),
        "MOD_LCTL" | "MOD_LCTRL" => "LCtrl".into(),
        "MOD_RCTL" | "MOD_RCTRL" => "RCtrl".into(),
        "MOD_LALT" => "LAlt".into(),
        "MOD_RALT" => "RAlt".into(),
        "MOD_LGUI" => "LGui".into(),
        "MOD_RGUI" => "RGui".into(),
        other => {
            // Pass through unknown modifier as-is
            other.to_string()
        }
    }
}

/// Map a QMK modifier combo like "MOD_LSFT | MOD_LGUI" to RMK format "LShift | LGui"
fn map_qmk_modifier_combo(s: &str) -> String {
    s.split('|')
        .map(|part| map_qmk_modifier(part.trim()))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Map QMK processor string to RMK chip hint.
/// Returns None if the processor has no RMK equivalent.
pub(crate) fn map_qmk_processor(processor: &str) -> Option<String> {
    let lower = processor.to_lowercase();
    if lower == "rp2040" {
        return Some("rp2040".into());
    }
    if lower.starts_with("stm32") {
        return Some(lower);
    }
    if lower.contains("nrf52840") {
        return Some("nrf52840".into());
    }
    if lower.contains("nrf52833") {
        return Some("nrf52833".into());
    }
    if lower.contains("nrf52832") {
        return Some("nrf52832".into());
    }
    // ATmega-based MCUs (Pro Micro, etc.) have no RMK equivalent
    if lower.starts_with("atmega") || lower.starts_with("at90") {
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_basic_letters() {
        assert_eq!(map_qmk_keycode("KC_A").unwrap(), "A");
        assert_eq!(map_qmk_keycode("KC_Z").unwrap(), "Z");
        // Case insensitive
        assert_eq!(map_qmk_keycode("kc_a").unwrap(), "A");
    }

    #[test]
    fn test_map_numbers() {
        assert_eq!(map_qmk_keycode("KC_1").unwrap(), "Kc1");
        assert_eq!(map_qmk_keycode("KC_0").unwrap(), "Kc0");
    }

    #[test]
    fn test_map_special_keys() {
        assert_eq!(map_qmk_keycode("KC_ESC").unwrap(), "Escape");
        assert_eq!(map_qmk_keycode("KC_ENT").unwrap(), "Enter");
        assert_eq!(map_qmk_keycode("KC_ENTER").unwrap(), "Enter");
        assert_eq!(map_qmk_keycode("KC_SPC").unwrap(), "Space");
        assert_eq!(map_qmk_keycode("KC_BSPC").unwrap(), "Backspace");
        assert_eq!(map_qmk_keycode("KC_TAB").unwrap(), "Tab");
        assert_eq!(map_qmk_keycode("KC_DEL").unwrap(), "Delete");
    }

    #[test]
    fn test_map_modifiers() {
        assert_eq!(map_qmk_keycode("KC_LSFT").unwrap(), "LShift");
        assert_eq!(map_qmk_keycode("KC_LCTL").unwrap(), "LCtrl");
        assert_eq!(map_qmk_keycode("KC_LALT").unwrap(), "LAlt");
        assert_eq!(map_qmk_keycode("KC_LGUI").unwrap(), "LGui");
        assert_eq!(map_qmk_keycode("KC_RSFT").unwrap(), "RShift");
        assert_eq!(map_qmk_keycode("KC_RCTL").unwrap(), "RCtrl");
        assert_eq!(map_qmk_keycode("KC_RALT").unwrap(), "RAlt");
        assert_eq!(map_qmk_keycode("KC_RGUI").unwrap(), "RGui");
    }

    #[test]
    fn test_map_navigation() {
        assert_eq!(map_qmk_keycode("KC_UP").unwrap(), "Up");
        assert_eq!(map_qmk_keycode("KC_DOWN").unwrap(), "Down");
        assert_eq!(map_qmk_keycode("KC_LEFT").unwrap(), "Left");
        assert_eq!(map_qmk_keycode("KC_RGHT").unwrap(), "Right");
        assert_eq!(map_qmk_keycode("KC_HOME").unwrap(), "Home");
        assert_eq!(map_qmk_keycode("KC_END").unwrap(), "End");
        assert_eq!(map_qmk_keycode("KC_PGUP").unwrap(), "PageUp");
        assert_eq!(map_qmk_keycode("KC_PGDN").unwrap(), "PageDown");
    }

    #[test]
    fn test_map_function_keys() {
        assert_eq!(map_qmk_keycode("KC_F1").unwrap(), "F1");
        assert_eq!(map_qmk_keycode("KC_F12").unwrap(), "F12");
        assert_eq!(map_qmk_keycode("KC_F24").unwrap(), "F24");
    }

    #[test]
    fn test_map_transparent_and_no() {
        assert_eq!(map_qmk_keycode("KC_TRNS").unwrap(), "_");
        assert_eq!(map_qmk_keycode("KC_TRANSPARENT").unwrap(), "_");
        assert_eq!(map_qmk_keycode("_______").unwrap(), "_");
        assert_eq!(map_qmk_keycode("KC_NO").unwrap(), "No");
        assert_eq!(map_qmk_keycode("XXXXXXX").unwrap(), "No");
    }

    #[test]
    fn test_map_layer_operations() {
        assert_eq!(map_qmk_keycode("MO(1)").unwrap(), "MO(1)");
        assert_eq!(map_qmk_keycode("TG(2)").unwrap(), "TG(2)");
        assert_eq!(map_qmk_keycode("TT(1)").unwrap(), "TT(1)");
        assert_eq!(map_qmk_keycode("OSL(3)").unwrap(), "OSL(3)");
        assert_eq!(map_qmk_keycode("DF(0)").unwrap(), "DF(0)");
        assert_eq!(map_qmk_keycode("TO(1)").unwrap(), "TO(1)");
    }

    #[test]
    fn test_map_lt() {
        assert_eq!(
            map_qmk_keycode("LT(1, KC_SPC)").unwrap(),
            "LT(1, Space)"
        );
        assert_eq!(
            map_qmk_keycode("LT(2, KC_ESC)").unwrap(),
            "LT(2, Escape)"
        );
    }

    #[test]
    fn test_map_lm() {
        assert_eq!(
            map_qmk_keycode("LM(1, MOD_LSFT)").unwrap(),
            "LM(1, LShift)"
        );
        assert_eq!(
            map_qmk_keycode("LM(1, MOD_LSFT | MOD_LGUI)").unwrap(),
            "LM(1, LShift | LGui)"
        );
    }

    #[test]
    fn test_map_osm() {
        assert_eq!(map_qmk_keycode("OSM(MOD_LSFT)").unwrap(), "OSM(LShift)");
    }

    #[test]
    fn test_map_media() {
        assert_eq!(map_qmk_keycode("KC_MPLY").unwrap(), "MediaPlayPause");
        assert_eq!(map_qmk_keycode("KC_VOLU").unwrap(), "AudioVolUp");
        assert_eq!(map_qmk_keycode("KC_VOLD").unwrap(), "AudioVolDown");
        assert_eq!(map_qmk_keycode("KC_MUTE").unwrap(), "AudioMute");
        assert_eq!(map_qmk_keycode("KC_MNXT").unwrap(), "MediaNextTrack");
        assert_eq!(map_qmk_keycode("KC_MPRV").unwrap(), "MediaPrevTrack");
    }

    #[test]
    fn test_map_unknown_keycode() {
        assert!(map_qmk_keycode("QK_BOOT").is_err());
        assert!(map_qmk_keycode("RGB_TOG").is_err());
        assert!(map_qmk_keycode("LCTL_T(KC_A)").is_err());
    }

    #[test]
    fn test_map_processor() {
        assert_eq!(map_qmk_processor("RP2040"), Some("rp2040".into()));
        assert_eq!(
            map_qmk_processor("STM32F303"),
            Some("stm32f303".into())
        );
        assert_eq!(
            map_qmk_processor("nRF52840"),
            Some("nrf52840".into())
        );
        assert_eq!(map_qmk_processor("atmega32u4"), None);
        assert_eq!(map_qmk_processor("at90usb1286"), None);
    }
}
