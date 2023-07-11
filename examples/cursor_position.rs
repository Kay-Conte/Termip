use std::io::{stdin, stdout};

use termip::terminal::{enable_raw_mode, erase_entire_screen, get_cursor_position, move_cursor};

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    let mut inp = stdin();

    enable_raw_mode(&mut inp)?;
    erase_entire_screen(&mut out)?;
    move_cursor(&mut out, 1, 1)?;

    let (x, y) = get_cursor_position(&mut out, &mut inp)?.expect("Should always return a position");

    println!("X: {} Y: {}", x, y);

    Ok(())
}
