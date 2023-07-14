use std::io::{stdout, Write};

use termip::{utils::set_fg, style::Color};

fn main() -> std::io::Result<()> {
    let mut out = stdout();

    set_fg(&mut out, Color::Red)?;
    write!(out, "Red\n")?;

    set_fg(&mut out, Color::Yellow)?;
    write!(out, "Yellow\n")?;

    set_fg(&mut out, Color::Green)?;
    write!(out, "Green\n")?;

    // Reset as to not effect other applications after closing
    set_fg(&mut out, Color::Reset)?;

    Ok(())
}
