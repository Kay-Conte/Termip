use std::io::stdout;

use termip::utils::get_size;

fn main() -> std::io::Result<()> {
    let out = stdout();

    let s = get_size(&out)?;

    println!("{}:{}", s.0, s.1);

    Ok(())
}
