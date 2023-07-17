use std::io::{stdin, stdout, Write};

use termip::{utils::{enable_raw_mode, read_batch, disable_raw_mode}, events::KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = stdin();
    let mut out = stdout();

    enable_raw_mode(&mut inp)?;

    out.flush()?;

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
