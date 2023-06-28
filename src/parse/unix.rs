use crate::events::{EventBatch, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn parse_partial_buffer<I>(mut sequence: I) -> Option<Event> where I: Iterator<Item = u8> {
    match sequence.next() {
        Some(b'\x1b') => {
            match sequence.next() {
                Some(b'[') => {
                    match sequence.next() {
                        Some(b'A') => Some(KeyCode::UpArrow.into()), 
                        Some(b'B') => Some(KeyCode::DownArrow.into()), 
                        Some(b'C') => Some(KeyCode::RightArrow.into()), 
                        Some(b'D') => Some(KeyCode::LeftArrow.into()), 

                        Some(b'F') => Some(KeyCode::End.into()),
                        Some(b'H') => Some(KeyCode::Home.into()),

                        Some(b'Z') => Some(Event::Key(KeyEvent {
                            code: KeyCode::BackTab,
                            modifiers: KeyModifiers::Shift,
                        })),

                        Some(b'M') => unimplemented!("Mouse not handled yet"),
                        Some(b'<') => unimplemented!("Mouse not handled yet"),

                        Some(b'I') => Some(Event::FocusGained),
                        Some(b'O') => Some(Event::FocusLost),
                        
                        Some(b) => Some(Event::UnhandledControlSequence(format!("\x1b{b}"))),
                        None => Some(Event::UnhandledControlSequence("\x1b".to_string())),
                    }
                }
                Some(b'\x1b') => Some(KeyCode::Escape.into()),

                Some(b) => Some(Event::UnhandledControlSequence(format!("\x1b{b}"))),
                None => Some(Event::UnhandledControlSequence("\x1b".to_string())),
            }
        }
        Some(c) => char::from_u32(c as u32).map(|c| KeyCode::Char(c).into()),
        _ => None,
    }
}

pub fn parse_entire_buffer(sequence: Vec<u8>) -> EventBatch {
    let mut batch: Vec<Event> = Vec::new();
    let mut bytes = sequence.into_iter();

    loop {
        if let Some(e) = parse_partial_buffer(&mut bytes) {
            batch.push(e);
        } else {
            break;
        }
    }
    
    batch.into()
}
