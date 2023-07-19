#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "windows")]
pub mod windows;

use std::vec::IntoIter;

#[derive(Debug, Clone)]
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

    pub fn iter<'a>(&'a self) -> EventBatchIter<'a> {
        EventBatchIter::new(&self.internal)
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

pub struct EventBatchIter<'a> {
    internal: &'a Vec<Event>,
    idx: usize,
}

impl<'a> EventBatchIter<'a> {
    pub fn new(internal: &'a Vec<Event>) -> Self {
        Self {
            internal,
            idx: 0,
        }
    }
}

impl<'a> Iterator for EventBatchIter<'a> {
    type Item = &'a Event;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.internal.get(self.idx);

        self.idx += 1;

        item
    }
}

#[repr(u8)]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum KeyModifiers {
    None = 0b0000_0000,
    Shift = 0b0000_0001,
    Control = 0b0000_0010,
    Alt = 0b0000_0100,
    Super = 0b0000_1000,
    Hyper = 0b0001_0000,
    Meta = 0b0010_0000,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,

    pub modifiers: KeyModifiers,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl Event {
    /// Returns whether or not a given keycode was pressed, this is used to simplify the interface
    /// of matching simple events
    pub fn pressed(&self, key_code: KeyCode) -> bool {
        match self {
            Event::Key(KeyEvent { code, .. }) => *code == key_code,
            _ => false,
        }
    }

    /// Returns whether or not a givent key event occured, this is used to simplify the interface
    /// of matching simple events
    pub fn pressed_modified(&self, key_event: KeyEvent) -> bool {
        match self {
            Event::Key(event) => *event == key_event,
            _ => false,
        }
    }
}
