use std::vec::IntoIter;

#[derive(Debug)]
pub struct EventBatch {
    internal: Vec<Event>,
}

impl EventBatch {
    pub fn empty() -> Self {
        Self {
            internal: Vec::new(),
        }
    }

    pub fn pressed(&self, target: KeyCode) -> bool {
        self.internal.iter().any(|i| match i {
            Event::Key(KeyEvent { code, .. }) => *code == target,
            _ => false,
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

    F(u8),
}

#[derive(Debug)]
pub struct KeyEvent {
    pub code: KeyCode,

    pub modifiers: KeyModifiers,
}

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),

    FocusGained,
    FocusLost,

    Cursor(u16, u16),

    UnrecognizedControlSequence,
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
