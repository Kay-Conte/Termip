use std::io::stdin;

use termip::{utils::{read_single, enable_raw_mode, disable_raw_mode}, events::KeyCode};

fn main() -> std::io::Result<()>{
    let mut inp = stdin();

    enable_raw_mode(&mut inp)?;

    loop {
        let ev = read_single(&mut inp)?.expect("input has been set to nonblocking");

        if ev.pressed(KeyCode::Char('q')) { break; }

        dbg!(ev);
    }

    disable_raw_mode(&mut inp)?;

    Ok(())
}
