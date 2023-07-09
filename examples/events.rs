use std::{io::stdin, time::Duration};

use termip::{
    events::{Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, try_read_batch},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut inp = stdin();

    enable_raw_mode(&mut inp)?;

    loop {
        let batch = try_read_batch(&mut inp)?;
        
        if batch.pressed(KeyCode::RightArrow) {
            println!("Left arrow is pressed");
        }

        for event in batch.into_iter() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    println!("{c}");
                }
                Event::Key(KeyEvent {
                    code: KeyCode::LeftArrow,
                    ..
                }) => {
                    println!("Left arrow");
                }
                Event::UnhandledControlSequence(c) => {
                    println!("Unrecognized control sequence {:?}", c.bytes());
                }
                _ => {
                    println!("Unhandled event");
                }
            }
        }

        // about 60 fps without inter frame timing
        std::thread::sleep(Duration::from_millis(16));
    }
}
