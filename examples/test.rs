use std::{io::stdin, time::Duration};

use termip::{events::Event, terminal::{try_read_event, enable_raw_mode}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = stdin();

    enable_raw_mode(&mut s)?;

    

    loop {
        // about 60 fps without timing
        std::thread::sleep(Duration::from_millis(1000));

        let Some(event) = try_read_event(&mut s).unwrap() else {
            println!("Proof that this loop is not being blocked like stupid bitch boy does");
            continue; 
        };

        match event {
            Event::Char('a') => {
                println!("A");
            }
            Event::LeftArrow => {
                println!("Left arrow");
            }
            Event::RightArrow => {
                println!("Right arrow");
            }
            Event::UpArrow => {
                println!("Up arrow");
            }
            Event::DownArrow=> {
                println!("Down arrow");
            }
            Event::End => {
                println!("End"); 
            }
            _ => { println!("Unhandled event"); }
        }


    }
}
