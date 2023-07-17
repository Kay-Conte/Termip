use std::{
    io::{stdin, stdout, Write},
    time::Duration,
};

use termip::utils::{
    self, enable_raw_mode, hide_cursor, move_cursor, show_cursor, disable_raw_mode, enter_alternate_buffer, leave_alternate_buffer,
};

fn main() -> std::io::Result<()> {
    let mut inp = stdin();
    let mut out = stdout();

    enter_alternate_buffer(&mut out)?;
    enable_raw_mode(&mut inp)?;

    let (x, y) = utils::get_size(&out)?;

    let x = x / 2;
    let y = y / 2;

    move_cursor(&mut out, x, y)?;

    hide_cursor(&mut out)?;

    out.flush()?;

    std::thread::sleep(Duration::from_secs(1));

    show_cursor(&mut out)?;

    out.flush()?;

    std::thread::sleep(Duration::from_secs(1));

    move_cursor(&mut out, 10, 10)?;
    
    out.flush()?;

    std::thread::sleep(Duration::from_secs(1));

    show_cursor(&mut out)?;
    move_cursor(&mut out, 0, 0)?;
    disable_raw_mode(&mut inp)?;
    leave_alternate_buffer(&mut out)?;

    Ok(())
}
