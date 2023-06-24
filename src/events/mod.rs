use std::marker::PhantomData;

pub enum MouseButton {
    Left,
}

pub enum MouseEvent {
    Button(MouseButton),
}

impl MouseEvent {
    fn test() -> Self {
        Self::Button(MouseButton::Left)
    }
}

#[cfg(not(windows))]
impl MouseEvent {
    /// Function assume 5 byte mouse event
    pub fn try_from_slice(bytes: &[u8]) -> Option<Self> {
        unimplemented!()
    }
}

pub enum Event {
    Char(char),

    Backspace,
    Tab,
    Enter,
    Shift,
    Control,
    Alt,
    CapsLock,
    Escape,
    End,

    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,

    Mouse(MouseEvent),

    UnhandledControl(char),
    OutOfRange,
}

pub struct BatchIter {
    internal: std::vec::IntoIter<Event>,
}

impl BatchIter {
    fn new<I>(items: I) -> Self
    where
        I: IntoIterator<Item = Event>,
    {
        Self {
            internal: Vec::from_iter(items).into_iter(),
        }
    }
}

impl Iterator for BatchIter {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}

#[cfg(not(windows))]
impl Event {
    pub fn try_from_byte_pair(pair: u16) -> Option<Self> {
        char::from_u32(pair as u32).map(|c| Self::Char(c))
    }

    pub fn try_from_str(sequence: &str) -> Option<Self> {
        if sequence.len() == 1 {
            return Some(Event::Char(sequence.chars().next().expect(
                "Sequence should have at least one char to pass the clause",
            )));
        }

        match sequence {
            "\x1b[D" => Some(Event::LeftArrow),
            "\x1b[C" => Some(Event::RightArrow),
            "\x1b[A" => Some(Event::UpArrow),
            "\x1b[B" => Some(Event::DownArrow),
            "\x1b[F" => Some(Event::End),
            _ => None,
        }
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
