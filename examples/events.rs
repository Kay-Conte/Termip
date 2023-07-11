use std::io::stdin;

use termip::terminal::{enable_raw_mode, read_batch};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = stdin();

    enable_raw_mode(&mut inp)?;

    loop {
        let batch = read_batch(&mut inp)?;

        for event in batch.into_iter() {
            println!("{:?}", event);
        }
    }
}
