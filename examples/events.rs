use std::io::{stdin, stdout};

use termip::{terminal::{enable_raw_mode, read_batch, platform::request_cursor_position, disable_raw_mode}, events::KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = stdin();
    let mut out = stdout();

    enable_raw_mode(&mut inp)?;

    request_cursor_position(&mut out)?;

    loop {
        let batch = read_batch(&mut inp)?;

        if batch.pressed(KeyCode::Char('c')) {
            break;
        }

        for event in batch {
            println!("{:?}", event);
        }
    }

    disable_raw_mode(&mut inp)?;

    Ok(())
}
