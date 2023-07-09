use std::{io::stdout, time::Duration};

use termip::terminal::platform::{hide_cursor, show_cursor};


fn main() -> std::io::Result<()> {
    let mut out = stdout();

    hide_cursor(&mut out)?;

    std::thread::sleep(Duration::from_secs(1));

    show_cursor(&mut out)?;

    Ok(())
}
