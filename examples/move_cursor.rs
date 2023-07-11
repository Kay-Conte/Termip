use std::{
    io::{stdin, stdout, Stdout, Write},
    time::Duration,
};

use termip::terminal::{enable_raw_mode, erase_entire_screen, move_cursor, size};

fn move_and_wait(s: &mut Stdout, line: u16, column: u16) -> std::io::Result<()> {
    move_cursor(s, line, column)?;

    s.flush()?;

    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    let mut inp = stdin();

    erase_entire_screen(&mut out)?;

    move_cursor(&mut out, 0, 0)?;

    enable_raw_mode(&mut inp)?;

    out.flush()?;

    loop {
        let size = size(&out)?;

        move_and_wait(&mut out, 0, 0)?;

        move_and_wait(&mut out, size.0, 0)?;

        move_and_wait(&mut out, size.0, size.1)?;

        move_and_wait(&mut out, 0, size.1)?;
    }
}
