use std::io::stdin;

use termip::terminal::{platform::read_single, enable_raw_mode, disable_raw_mode};

fn main() -> std::io::Result<()> {
    let mut stdin = stdin();
    
    enable_raw_mode(&mut stdin)?;

    let e = read_single(&mut stdin)?;

    println!("{:?}", e);

    disable_raw_mode(&mut stdin)?;

    Ok(())
}
