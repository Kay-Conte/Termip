use std::io::stdout;

use termip::terminal::size;

fn main() -> std::io::Result<()> {
    let out = stdout();

    let s = size(&out)?;

    println!("{}:{}", s.0, s.1);

    Ok(())
}
