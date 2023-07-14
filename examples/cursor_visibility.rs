use std::{
    io::{stdin, stdout, Write},
    time::Duration,
};

use termip::terminal::{
    self, enable_raw_mode, erase_entire_screen, hide_cursor, move_cursor, show_cursor, disable_raw_mode,
};

fn main() -> std::io::Result<()> {
    let mut inp = stdin();
    let mut out = stdout();

    enable_raw_mode(&mut inp)?;
    erase_entire_screen(&mut out)?;

    let (x, y) = terminal::get_size(&out)?;

    let x = x / 2;
    let y = y / 2;

    move_cursor(&mut out, x, y)?;

    for _ in 0..5 {
        show_cursor(&mut out)?;

        out.flush()?;

        std::thread::sleep(Duration::from_secs(1));

        hide_cursor(&mut out)?;

        out.flush()?;

        std::thread::sleep(Duration::from_secs(1));
    }

    show_cursor(&mut out)?;
    move_cursor(&mut out, 0, 0)?;
    disable_raw_mode(&mut inp)?;

    Ok(())
}
