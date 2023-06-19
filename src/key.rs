pub enum Key {
    Char(char),

    Backspace,
    Tab,
    Enter,
    Shift,
    Control,
    Alt,
    CapsLock,
    Escape,
    Space,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    UnhandledControl(char),
    OutOfRange,
}

#[cfg(target_os = "windows")]
mod platform {
    use winapi::um::wincontypes::KEY_EVENT_RECORD;

    use super::Key;

    impl Key {
        fn from_virtual_code(virtual_code: u16) -> Self {
            match virtual_code {
                8 => Key::Backspace,
                9 => Key::Tab,
                13 => Key::Enter,
                16 => Key::Shift,
                17 => Key::Control,
                18 => Key::Alt,
                20 => Key::CapsLock,
                27 => Key::Escape,
                32 => Key::Space,
                37 => Key::LeftArrow,
                39 => Key::RightArrow,
                38 => Key::UpArrow,
                40 => Key::DownArrow,
                0..=31 | 33..=255 => Key::UnhandledControl(virtual_code as u8 as char),
                _ => Key::OutOfRange,
            }
        }

        fn from_printable(code: u16) -> Self {
            Key::Char(char::from_u32(code as u32).expect("Invalid char received"))
        }
    }

    impl From<KEY_EVENT_RECORD> for Key {
        fn from(key: KEY_EVENT_RECORD) -> Key {
            if unsafe { *key.uChar.UnicodeChar() } == 0 {
                Key::from_virtual_code(key.wVirtualKeyCode)
            } else {
                unsafe { Key::from_printable(*key.uChar.UnicodeChar()) }
            }
        }
    }
}
