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

// ── ZMK Mappings ──────────────────────────────────────────────────────────

/// Static mapping from ZMK keycode names to RMK keycode names.
/// ZMK keys are stored in UPPERCASE for case-insensitive lookup.
static ZMK_TO_RMK: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();

    // Letters — ZMK uses bare names: A, B, C, ...
    for c in b'A'..=b'Z' {
        let s: &'static str = match c {
            b'A' => "A", b'B' => "B", b'C' => "C", b'D' => "D", b'E' => "E",
            b'F' => "F", b'G' => "G", b'H' => "H", b'I' => "I", b'J' => "J",
            b'K' => "K", b'L' => "L", b'M' => "M", b'N' => "N", b'O' => "O",
            b'P' => "P", b'Q' => "Q", b'R' => "R", b'S' => "S", b'T' => "T",
            b'U' => "U", b'V' => "V", b'W' => "W", b'X' => "X", b'Y' => "Y",
            b'Z' => "Z", _ => unreachable!(),
        };
        m.insert(s, s);
    }

    // Numbers — ZMK uses N1..N0
    m.insert("N1", "Kc1");
    m.insert("N2", "Kc2");
    m.insert("N3", "Kc3");
    m.insert("N4", "Kc4");
    m.insert("N5", "Kc5");
    m.insert("N6", "Kc6");
    m.insert("N7", "Kc7");
    m.insert("N8", "Kc8");
    m.insert("N9", "Kc9");
    m.insert("N0", "Kc0");
    m.insert("NUMBER_1", "Kc1");
    m.insert("NUMBER_2", "Kc2");
    m.insert("NUMBER_3", "Kc3");
    m.insert("NUMBER_4", "Kc4");
    m.insert("NUMBER_5", "Kc5");
    m.insert("NUMBER_6", "Kc6");
    m.insert("NUMBER_7", "Kc7");
    m.insert("NUMBER_8", "Kc8");
    m.insert("NUMBER_9", "Kc9");
    m.insert("NUMBER_0", "Kc0");

    // Special keys
    m.insert("RET", "Enter");
    m.insert("RETURN", "Enter");
    m.insert("ENTER", "Enter");
    m.insert("ESC", "Escape");
    m.insert("ESCAPE", "Escape");
    m.insert("BSPC", "Backspace");
    m.insert("BACKSPACE", "Backspace");
    m.insert("TAB", "Tab");
    m.insert("SPACE", "Space");
    m.insert("SPC", "Space");
    m.insert("MINUS", "Minus");
    m.insert("EQUAL", "Equal");
    m.insert("LBKT", "LeftBracket");
    m.insert("LEFT_BRACKET", "LeftBracket");
    m.insert("RBKT", "RightBracket");
    m.insert("RIGHT_BRACKET", "RightBracket");
    m.insert("BSLH", "Backslash");
    m.insert("BACKSLASH", "Backslash");
    m.insert("NON_US_HASH", "NonusHash");
    m.insert("SEMI", "Semicolon");
    m.insert("SEMICOLON", "Semicolon");
    m.insert("SQT", "Quote");
    m.insert("SINGLE_QUOTE", "Quote");
    m.insert("APOSTROPHE", "Quote");
    m.insert("APOS", "Quote");
    m.insert("GRAVE", "Grave");
    m.insert("COMMA", "Comma");
    m.insert("DOT", "Dot");
    m.insert("PERIOD", "Dot");
    m.insert("FSLH", "Slash");
    m.insert("SLASH", "Slash");
    m.insert("CAPS", "CapsLock");
    m.insert("CAPSLOCK", "CapsLock");
    m.insert("CLCK", "CapsLock");

    // Function keys
    m.insert("F1", "F1"); m.insert("F2", "F2"); m.insert("F3", "F3");
    m.insert("F4", "F4"); m.insert("F5", "F5"); m.insert("F6", "F6");
    m.insert("F7", "F7"); m.insert("F8", "F8"); m.insert("F9", "F9");
    m.insert("F10", "F10"); m.insert("F11", "F11"); m.insert("F12", "F12");
    m.insert("F13", "F13"); m.insert("F14", "F14"); m.insert("F15", "F15");
    m.insert("F16", "F16"); m.insert("F17", "F17"); m.insert("F18", "F18");
    m.insert("F19", "F19"); m.insert("F20", "F20"); m.insert("F21", "F21");
    m.insert("F22", "F22"); m.insert("F23", "F23"); m.insert("F24", "F24");

    // Navigation
    m.insert("PSCRN", "PrintScreen");
    m.insert("PRINTSCREEN", "PrintScreen");
    m.insert("SLCK", "ScrollLock");
    m.insert("SCROLLLOCK", "ScrollLock");
    m.insert("PAUSE_BREAK", "Pause");
    m.insert("INS", "Insert");
    m.insert("INSERT", "Insert");
    m.insert("HOME", "Home");
    m.insert("PG_UP", "PageUp");
    m.insert("PAGE_UP", "PageUp");
    m.insert("DEL", "Delete");
    m.insert("DELETE", "Delete");
    m.insert("END", "End");
    m.insert("PG_DN", "PageDown");
    m.insert("PAGE_DOWN", "PageDown");
    m.insert("RIGHT", "Right");
    m.insert("LEFT", "Left");
    m.insert("DOWN", "Down");
    m.insert("UP", "Up");

    // Modifiers
    m.insert("LSHFT", "LShift"); m.insert("LSHIFT", "LShift"); m.insert("LEFT_SHIFT", "LShift");
    m.insert("RSHFT", "RShift"); m.insert("RSHIFT", "RShift"); m.insert("RIGHT_SHIFT", "RShift");
    m.insert("LCTRL", "LCtrl"); m.insert("LEFT_CONTROL", "LCtrl"); m.insert("LCTL", "LCtrl");
    m.insert("RCTRL", "RCtrl"); m.insert("RIGHT_CONTROL", "RCtrl"); m.insert("RCTL", "RCtrl");
    m.insert("LALT", "LAlt"); m.insert("LEFT_ALT", "LAlt");
    m.insert("RALT", "RAlt"); m.insert("RIGHT_ALT", "RAlt");
    m.insert("LGUI", "LGui"); m.insert("LEFT_GUI", "LGui"); m.insert("LWIN", "LGui"); m.insert("LCMD", "LGui"); m.insert("LMETA", "LGui");
    m.insert("RGUI", "RGui"); m.insert("RIGHT_GUI", "RGui"); m.insert("RWIN", "RGui"); m.insert("RCMD", "RGui"); m.insert("RMETA", "RGui");

    // Numpad
    m.insert("KP_NUM", "NumLock"); m.insert("KLCK", "NumLock");
    m.insert("KP_SLASH", "KpSlash"); m.insert("KP_DIVIDE", "KpSlash");
    m.insert("KP_MULTIPLY", "KpAsterisk"); m.insert("KP_ASTERISK", "KpAsterisk");
    m.insert("KP_MINUS", "KpMinus"); m.insert("KP_SUBTRACT", "KpMinus");
    m.insert("KP_PLUS", "KpPlus"); m.insert("KP_ENTER", "KpEnter");
    m.insert("KP_N1", "Kp1"); m.insert("KP_NUMBER_1", "Kp1");
    m.insert("KP_N2", "Kp2"); m.insert("KP_NUMBER_2", "Kp2");
    m.insert("KP_N3", "Kp3"); m.insert("KP_NUMBER_3", "Kp3");
    m.insert("KP_N4", "Kp4"); m.insert("KP_NUMBER_4", "Kp4");
    m.insert("KP_N5", "Kp5"); m.insert("KP_NUMBER_5", "Kp5");
    m.insert("KP_N6", "Kp6"); m.insert("KP_NUMBER_6", "Kp6");
    m.insert("KP_N7", "Kp7"); m.insert("KP_NUMBER_7", "Kp7");
    m.insert("KP_N8", "Kp8"); m.insert("KP_NUMBER_8", "Kp8");
    m.insert("KP_N9", "Kp9"); m.insert("KP_NUMBER_9", "Kp9");
    m.insert("KP_N0", "Kp0"); m.insert("KP_NUMBER_0", "Kp0");
    m.insert("KP_DOT", "KpDot"); m.insert("KP_EQUAL", "KpEqual");

    // Note: ZMK shifted symbols (EXCL, AT, HASH, etc.) are intentionally
    // NOT mapped here. They represent Shift+BaseKey combos (e.g. EXCL = LS(N1))
    // which have no single-key equivalent in RMK. They will produce a clear
    // warning directing the user to use the base key + modifier instead.

    // Media keys
    m.insert("C_MUTE", "AudioMute");
    m.insert("C_VOL_UP", "AudioVolUp"); m.insert("C_VOLUME_UP", "AudioVolUp");
    m.insert("C_VOL_DN", "AudioVolDown"); m.insert("C_VOLUME_DOWN", "AudioVolDown");
    m.insert("C_NEXT", "MediaNextTrack");
    m.insert("C_PREV", "MediaPrevTrack"); m.insert("C_PREVIOUS", "MediaPrevTrack");
    m.insert("C_STOP", "MediaStop");
    m.insert("C_PP", "MediaPlayPause"); m.insert("C_PLAY_PAUSE", "MediaPlayPause");
    m.insert("C_BRI_UP", "BrightnessUp"); m.insert("C_BRIGHTNESS_INC", "BrightnessUp");
    m.insert("C_BRI_DN", "BrightnessDown"); m.insert("C_BRIGHTNESS_DEC", "BrightnessDown");

    m
});

/// ZMK shifted symbol names that need Shift+BaseKey in RMK.
/// Maps symbol name → human-readable hint for the warning message.
static ZMK_SHIFTED_SYMBOLS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("EXCL", "! (use Shift + Kc1)");
    m.insert("EXCLAMATION", "! (use Shift + Kc1)");
    m.insert("AT", "@ (use Shift + Kc2)");
    m.insert("AT_SIGN", "@ (use Shift + Kc2)");
    m.insert("HASH", "# (use Shift + Kc3)");
    m.insert("POUND", "# (use Shift + Kc3)");
    m.insert("DLLR", "$ (use Shift + Kc4)");
    m.insert("DOLLAR", "$ (use Shift + Kc4)");
    m.insert("PRCNT", "% (use Shift + Kc5)");
    m.insert("PERCENT", "% (use Shift + Kc5)");
    m.insert("CARET", "^ (use Shift + Kc6)");
    m.insert("AMPS", "& (use Shift + Kc7)");
    m.insert("AMPERSAND", "& (use Shift + Kc7)");
    m.insert("ASTRK", "* (use Shift + Kc8)");
    m.insert("ASTERISK", "* (use Shift + Kc8)");
    m.insert("STAR", "* (use Shift + Kc8)");
    m.insert("LPAR", "( (use Shift + Kc9)");
    m.insert("LEFT_PARENTHESIS", "( (use Shift + Kc9)");
    m.insert("RPAR", ") (use Shift + Kc0)");
    m.insert("RIGHT_PARENTHESIS", ") (use Shift + Kc0)");
    m.insert("UNDER", "_ (use Shift + Minus)");
    m.insert("UNDERSCORE", "_ (use Shift + Minus)");
    m.insert("PLUS", "+ (use Shift + Equal)");
    m.insert("LBRC", "{ (use Shift + LeftBracket)");
    m.insert("LEFT_BRACE", "{ (use Shift + LeftBracket)");
    m.insert("RBRC", "} (use Shift + RightBracket)");
    m.insert("RIGHT_BRACE", "} (use Shift + RightBracket)");
    m.insert("PIPE", "| (use Shift + Backslash)");
    m.insert("TILDE", "~ (use Shift + Grave)");
    m
});

/// Map a single ZMK keycode (the argument to &kp) to an RMK keycode string.
pub(crate) fn map_zmk_keycode(zmk: &str) -> Result<String, String> {
    let trimmed = zmk.trim();
    let upper = trimmed.to_uppercase();

    if let Some(rmk) = ZMK_TO_RMK.get(upper.as_str()) {
        return Ok(rmk.to_string());
    }

    // Check if it's a shifted symbol with a specific hint
    if let Some(hint) = ZMK_SHIFTED_SYMBOLS.get(upper.as_str()) {
        return Err(format!(
            "ZMK shifted symbol '{}' = {} — RMK has no single keycode for this",
            trimmed, hint
        ));
    }

    Err(format!("Unmapped ZMK keycode: {}", trimmed))
}

/// Map a full ZMK behavior binding (e.g. "&kp TAB", "&mo 1") to an RMK keycode string.
///
/// Returns Err for unsupported or unmappable behaviors.
pub(crate) fn map_zmk_behavior(binding: &str) -> Result<String, String> {
    let parts: Vec<&str> = binding.trim().split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty binding".into());
    }

    let behavior = parts[0];
    match behavior {
        "&kp" => {
            if parts.len() < 2 {
                return Err(format!("&kp missing keycode: {}", binding));
            }
            map_zmk_keycode(parts[1])
        }
        "&mo" => {
            if parts.len() < 2 {
                return Err(format!("&mo missing layer: {}", binding));
            }
            Ok(format!("MO({})", parts[1]))
        }
        "&lt" => {
            if parts.len() < 3 {
                return Err(format!("&lt missing args: {}", binding));
            }
            let layer = parts[1];
            match map_zmk_keycode(parts[2]) {
                Ok(mapped) => Ok(format!("LT({}, {})", layer, mapped)),
                Err(e) => Err(format!("In &lt: {}", e)),
            }
        }
        "&tog" => {
            if parts.len() < 2 {
                return Err(format!("&tog missing layer: {}", binding));
            }
            Ok(format!("TG({})", parts[1]))
        }
        "&to" => {
            if parts.len() < 2 {
                return Err(format!("&to missing layer: {}", binding));
            }
            Ok(format!("TO({})", parts[1]))
        }
        "&sl" => {
            if parts.len() < 2 {
                return Err(format!("&sl missing layer: {}", binding));
            }
            Ok(format!("OSL({})", parts[1]))
        }
        "&sk" => {
            if parts.len() < 2 {
                return Err(format!("&sk missing modifier: {}", binding));
            }
            match map_zmk_keycode(parts[1]) {
                Ok(mapped) => Ok(format!("OSM({})", mapped)),
                Err(e) => Err(format!("In &sk: {}", e)),
            }
        }
        "&trans" => Ok("_".into()),
        "&none" => Ok("No".into()),
        "&mt" => {
            // Mod-tap — RMK has no direct equivalent keycode
            Err(format!(
                "Unsupported ZMK behavior (mod-tap): {}. RMK uses [behavior.tap_hold] config instead.",
                binding
            ))
        }
        "&bt" | "&rgb_ug" | "&ext_power" | "&out" | "&reset" | "&bootloader" | "&sys_reset" => {
            Err(format!(
                "Unsupported ZMK behavior (system/BT/RGB): {}",
                binding
            ))
        }
        _ => Err(format!("Unknown ZMK behavior: {}", binding)),
    }
}

/// Map ZMK modifier name to RMK modifier for &sk / OSM
fn _map_zmk_modifier(zmk_mod: &str) -> String {
    // Reuse the regular keycode map since ZMK modifiers use same names
    map_zmk_keycode(zmk_mod).unwrap_or_else(|_| zmk_mod.to_string())
}

// ── QMK Processor Mapping ──────────────────────────────────────────────

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

    // ── ZMK tests ──

    #[test]
    fn test_zmk_basic_keys() {
        assert_eq!(map_zmk_keycode("A").unwrap(), "A");
        assert_eq!(map_zmk_keycode("Z").unwrap(), "Z");
        assert_eq!(map_zmk_keycode("N1").unwrap(), "Kc1");
        assert_eq!(map_zmk_keycode("N0").unwrap(), "Kc0");
    }

    #[test]
    fn test_zmk_special_keys() {
        assert_eq!(map_zmk_keycode("RET").unwrap(), "Enter");
        assert_eq!(map_zmk_keycode("ESC").unwrap(), "Escape");
        assert_eq!(map_zmk_keycode("BSPC").unwrap(), "Backspace");
        assert_eq!(map_zmk_keycode("TAB").unwrap(), "Tab");
        assert_eq!(map_zmk_keycode("SPACE").unwrap(), "Space");
        assert_eq!(map_zmk_keycode("SEMI").unwrap(), "Semicolon");
        assert_eq!(map_zmk_keycode("SQT").unwrap(), "Quote");
        assert_eq!(map_zmk_keycode("COMMA").unwrap(), "Comma");
        assert_eq!(map_zmk_keycode("DOT").unwrap(), "Dot");
        assert_eq!(map_zmk_keycode("FSLH").unwrap(), "Slash");
    }

    #[test]
    fn test_zmk_modifiers() {
        assert_eq!(map_zmk_keycode("LSHFT").unwrap(), "LShift");
        assert_eq!(map_zmk_keycode("LCTRL").unwrap(), "LCtrl");
        assert_eq!(map_zmk_keycode("LGUI").unwrap(), "LGui");
        assert_eq!(map_zmk_keycode("RALT").unwrap(), "RAlt");
    }

    #[test]
    fn test_zmk_behavior_kp() {
        assert_eq!(map_zmk_behavior("&kp A").unwrap(), "A");
        assert_eq!(map_zmk_behavior("&kp TAB").unwrap(), "Tab");
        assert_eq!(map_zmk_behavior("&kp LSHFT").unwrap(), "LShift");
        assert_eq!(map_zmk_behavior("&kp N1").unwrap(), "Kc1");
    }

    #[test]
    fn test_zmk_behavior_layers() {
        assert_eq!(map_zmk_behavior("&mo 1").unwrap(), "MO(1)");
        assert_eq!(map_zmk_behavior("&tog 2").unwrap(), "TG(2)");
        assert_eq!(map_zmk_behavior("&to 0").unwrap(), "TO(0)");
        assert_eq!(map_zmk_behavior("&sl 1").unwrap(), "OSL(1)");
        assert_eq!(map_zmk_behavior("&lt 1 SPACE").unwrap(), "LT(1, Space)");
    }

    #[test]
    fn test_zmk_behavior_special() {
        assert_eq!(map_zmk_behavior("&trans").unwrap(), "_");
        assert_eq!(map_zmk_behavior("&none").unwrap(), "No");
        assert_eq!(map_zmk_behavior("&sk LSHFT").unwrap(), "OSM(LShift)");
    }

    #[test]
    fn test_zmk_behavior_unsupported() {
        assert!(map_zmk_behavior("&mt LCTRL A").is_err());
        assert!(map_zmk_behavior("&bt BT_CLR").is_err());
        assert!(map_zmk_behavior("&rgb_ug RGB_TOG").is_err());
    }

    #[test]
    fn test_zmk_shifted_symbols_produce_error() {
        // These should NOT silently map — they must produce errors with hints
        let result = map_zmk_keycode("EXCL");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Shift + Kc1"), "Error should hint Shift+Kc1: {}", err);

        let result = map_zmk_keycode("PIPE");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Shift + Backslash"));
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
