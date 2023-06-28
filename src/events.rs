use std::vec::IntoIter;

pub struct EventBatch {
    internal: Vec<Event>,
}

impl EventBatch {
    pub fn pressed(&self, target: KeyCode) -> bool {
        self.internal.iter().any(|i| {
            match i {
                Event::Key(KeyEvent { code, .. } ) => *code == target,
                _ => false,
            }
        }) 
    }
}

impl IntoIterator for EventBatch {
    type Item = Event;

    type IntoIter = IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        self.internal.into_iter()
    }
}

impl From<Vec<Event>> for EventBatch {
    fn from(internal: Vec<Event>) -> Self {
        Self { internal }
    }
}

#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyModifiers {
    None = 0b0000_0000,
    Shift = 0b0000_0001,
    Control = 0b0000_0010,
    Alt = 0b0000_0100,
    Super = 0b0000_1000,
    Hyper = 0b0001_0000,
    Meta = 0b0010_0000,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum KeyCode {
    Char(char),

    Backspace,
    BackTab,
    Tab,
    Enter,
    Shift,
    Control,
    Alt,
    CapsLock,
    Escape,
    End,
    Home,
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    F(usize),
}

pub struct KeyEvent {
    pub code: KeyCode,

    pub modifiers: KeyModifiers,
}

pub enum Event {
    Key(KeyEvent),

    FocusGained,
    FocusLost,

    UnhandledControlSequence(String),
    OutOfRange,
}

impl From<KeyCode> for Event {
    fn from(value: KeyCode) -> Self {
        Self::Key(KeyEvent {
            code: value,
            modifiers: KeyModifiers::None,
        })
    }
}

// #[cfg(windows)]
// impl Key {
//
//     fn from_virtual_code(virtual_code: u16) -> Self {
//         use winapi::um::wincontypes::KEY_EVENT_RECORD;
//
//         match virtual_code {
//             8 => Key::Backspace,
//             9 => Key::Tab,
//             13 => Key::Enter,
//             16 => Key::Shift,
//             17 => Key::Control,
//             18 => Key::Alt,
//             20 => Key::CapsLock,
//             27 => Key::Escape,
//             32 => Key::Space,
//             37 => Key::LeftArrow,
//             39 => Key::RightArrow,
//             38 => Key::UpArrow,
//             40 => Key::DownArrow,
//             0..=31 | 33..=255 => Key::UnhandledControl(virtual_code as u8 as char),
//             _ => Key::OutOfRange,
//         }
//     }
//
//     fn from_printable(code: u16) -> Self {
//         Key::Char(char::from_u32(code as u32).expect("Invalid char received"))
//     }
// }
//
// #[cfg(windows)]
// impl From<KEY_EVENT_RECORD> for Key {
//     fn from(key: KEY_EVENT_RECORD) -> Key {
//         if unsafe { *key.uChar.UnicodeChar() } == 0 {
//             Key::from_virtual_code(key.wVirtualKeyCode)
//         } else {
//             unsafe { Key::from_printable(*key.uChar.UnicodeChar()) }
//         }
//     }
// }
