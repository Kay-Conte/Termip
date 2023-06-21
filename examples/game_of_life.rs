use std::io::stdin;

use Termip::terminal::try_read_event;



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut s = stdin();

    loop {
        let event = try_read_event(&mut s);

        
    }    

    Ok(())
}
