use std::io::{stdin, stdout};

use termip::terminal::{enable_raw_mode, get_cursor_position, disable_raw_mode};

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    let mut inp = stdin();

    // Raw mode must be enabled to get cursor position on unix systems
    enable_raw_mode(&mut inp)?;

    // This function can but usually wont block up to 1 second. It will return none after 1 second
    // of not receiving a result on unix systems. This will never return none on windows targets
    let (x, y) = get_cursor_position(&mut out, &mut inp)?.expect("Should always return a position");

    println!("X: {} Y: {}", x, y);

    disable_raw_mode(&mut inp)?;

    Ok(())
}
