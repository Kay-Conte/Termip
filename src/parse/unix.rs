use crate::events::{EventBatch, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn parse_partial_buffer<I>(mut sequence: I) -> Option<Event> where I: Iterator<Item = u8> {
    let Some(byte) = sequence.next() else {
        return None;
    };

    match byte {
        b'\x1b' => {
            let Some(byte) = sequence.next() else {
                return Some(Event::UnhandledControlSequence("\x1b".to_string()));
            };

            match byte {
                b'[' => {
                    let Some(byte) = sequence.next() else {
                        return Some(Event::UnhandledControlSequence("\x1b[".to_string()));
                    };

                    match byte {
                        b'A' => Some(KeyCode::UpArrow.into()), 
                        b'B' => Some(KeyCode::DownArrow.into()), 
                        b'C' => Some(KeyCode::RightArrow.into()), 
                        b'D' => Some(KeyCode::LeftArrow.into()), 

                        b'F' => Some(KeyCode::End.into()),
                        b'H' => Some(KeyCode::Home.into()),

                        b'Z' => Some(Event::Key(KeyEvent {
                            code: KeyCode::BackTab,
                            modifiers: KeyModifiers::Shift,
                        })),

                        b'M' => unimplemented!("Mouse not handled yet"),
                        b'<' => unimplemented!("Mouse not handled yet"),

                        b'I' => Some(Event::FocusGained),
                        b'O' => Some(Event::FocusLost),
                        
                        val @ b'0'..=b'9' => {
                            let registers = vec![vec![val]];
                            
                            loop {
                                let Some(byte) = sequence.next() else {

                                    // TODO join all unhandled characters to this sequence. 
                                    return Some(Event::UnhandledControlSequence("\x1b".to_string()));
                                };
                            }
                        }

                        b => Some(Event::UnhandledControlSequence(format!("\x1b[{b}"))),
                    }
                }
                b'O' => {
                    let Some(byte) = sequence.next() else {
                        return Some(Event::UnhandledControlSequence("\x1bO".to_string()));
                    };

                    match byte {
                        b'D' => Some(KeyCode::LeftArrow.into()),
                        b'C' => Some(KeyCode::UpArrow.into()),
                        b'B' => Some(KeyCode::DownArrow.into()),
                        b'H' => Some(KeyCode::Home.into()),
                        b'F' => Some(KeyCode::End.into()),

                        // F1 - F4
                        val @ b'P'..=b'S' => Some(KeyCode::F(1 + val - b'p').into()),

                        b => Some(Event::UnhandledControlSequence(format!("\x1bO{b}"))),
                    }
                }

                b'\x1b' => Some(KeyCode::Escape.into()),
                
                

                b => Some(Event::UnhandledControlSequence(format!("\x1b{b}"))),
            }
        }
        c => char::from_u32(c as u32).map(|c| KeyCode::Char(c).into()),
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
