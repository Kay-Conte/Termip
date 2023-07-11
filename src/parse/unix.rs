use crate::events::{Event, EventBatch, KeyCode, KeyEvent, KeyModifiers};

pub fn parse_event<I>(mut bytes: I) -> Option<Event>
where
    I: Iterator<Item = u8>,
{
    let Some(byte) = bytes.next() else {
        return None;
    };

    match byte {
        b'\x1b' => Some(parse_esc(bytes)),
        c => char::from_u32(c as u32).map(|c| KeyCode::Char(c).into()),
    }
}

/// "\x1b"
pub fn parse_esc<I>(mut bytes: I) -> Event
where
    I: Iterator<Item = u8>,
{
    let Some(byte) = bytes.next() else {
        return Event::UnrecognizedControlSequence;
    };

    match byte {
        b'[' => parse_opening_bracket(bytes),
        b'O' => parse_opening_o(bytes),

        b'\x1b' => KeyCode::Escape.into(),

        _ => Event::UnrecognizedControlSequence,
    }
}

/// "\x1b["
pub fn parse_opening_bracket<I>(mut bytes: I) -> Event
where
    I: Iterator<Item = u8>,
{
    let Some(byte) = bytes.next() else { return Event::UnrecognizedControlSequence; };

    match byte {
        b'A' => KeyCode::UpArrow.into(),
        b'B' => KeyCode::DownArrow.into(),
        b'C' => KeyCode::RightArrow.into(),
        b'D' => KeyCode::LeftArrow.into(),

        b'F' => KeyCode::End.into(),
        b'H' => KeyCode::Home.into(),

        b'Z' => Event::Key(KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::Shift,
        }),

        b'M' => unimplemented!("Mouse not handled yet"),
        b'<' => unimplemented!("Mouse not handled yet"),

        b'I' => Event::FocusGained,
        b'O' => Event::FocusLost,

        val @ b'0'..=b'9' => parse_numerical(bytes, val),

        _ => Event::UnrecognizedControlSequence,
    }
}

/// "\x1bO"
pub fn parse_opening_o<I>(mut bytes: I) -> Event
where
    I: Iterator<Item = u8>,
{
    let Some(byte) = bytes.next() else {
        return Event::UnrecognizedControlSequence;
    };

    match byte {
        b'D' => KeyCode::LeftArrow.into(),
        b'C' => KeyCode::UpArrow.into(),
        b'B' => KeyCode::DownArrow.into(),
        b'H' => KeyCode::Home.into(),
        b'F' => KeyCode::End.into(),

        // F1 - F4
        val @ b'P'..=b'S' => KeyCode::F(1 + val - b'p').into(),

        _ => Event::UnrecognizedControlSequence,
    }
}

/// "\x1b[#"
pub fn parse_numerical<I>(mut bytes: I, val: u8) -> Event
where
    I: Iterator<Item = u8>,
{
    let mut registers = vec![vec![val]];

    let mut len = 4;
    let mut current_idx = 0;

    loop {
        len += 1;

        if len > 126 {
            break Event::UnrecognizedControlSequence;
        }

        let Some(byte) = bytes.next() else {
            return Event::UnrecognizedControlSequence; 
        };

        match byte {
            val @ b'0'..=b'9' => registers[current_idx].push(val),
            b';' => {
                registers.push(vec![]);
                current_idx += 1;
            }
            b'R' if registers.len() == 2 => {
                let Ok(Ok(x)) = String::from_utf8(registers.swap_remove(0)).map(|x| x.parse::<u16>()) else {
                    break Event::UnrecognizedControlSequence;
                };

                let Ok(Ok(y)) = String::from_utf8(registers.swap_remove(0)).map(|x| x.parse::<u16>()) else {
                    break Event::UnrecognizedControlSequence;
                };

                break Event::Cursor(x, y);
            }
            _ => break Event::UnrecognizedControlSequence,
        }
    }
}

pub fn parse_batch(sequence: Vec<u8>) -> EventBatch {
    let mut batch: Vec<Event> = Vec::new();
    let mut bytes = sequence.into_iter();

    loop {
        if let Some(e) = parse_event(&mut bytes) {
            batch.push(e);
        } else {
            break;
        }
    }

    batch.into()
}
