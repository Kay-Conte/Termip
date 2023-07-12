use std::io::{Error, ErrorKind, Write};

use termip::terminal::{
    erase_entire_screen, hide_cursor, move_cursor, platform::RawOs, show_cursor, get_cursor_position,
};

#[derive(Debug)]
pub enum ClearType {
    All,
    AfterCursor,
    BeforeCursor,
    CurrentLine,
    UntilNewLine,
}

pub struct Rect;
pub struct Cell;

pub struct Termip<Out>
where
    Out: RawOs + Write,
{
    out: Out,
}

pub trait Backend {
    /// Draw the given content to the terminal screen.
    ///
    /// The content is provided as an iterator over `(u16, u16, &Cell)` tuples,
    /// where the first two elements represent the x and y coordinates, and the
    /// third element is a reference to the [`Cell`] to be drawn.
    fn draw<'a, I>(&mut self, content: I) -> Result<(), Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;

    /// Insert `n` line breaks to the terminal screen.
    ///
    /// This method is optional and may not be implemented by all backends.
    fn append_lines(&mut self, _n: u16) -> Result<(), Error> {
        Ok(())
    }

    /// Hide the cursor on the terminal screen.
    fn hide_cursor(&mut self) -> Result<(), Error>;

    /// Show the cursor on the terminal screen.
    fn show_cursor(&mut self) -> Result<(), Error>;

    /// Get the current cursor position on the terminal screen.
    fn get_cursor(&mut self) -> Result<(u16, u16), Error>;

    /// Set the cursor position on the terminal screen to the given x and y coordinates.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Error>;

    /// Clears the whole terminal screen
    fn clear(&mut self) -> Result<(), Error>;

    /// Clears a specific region of the terminal specified by the [`ClearType`] parameter
    ///
    /// This method is optional and may not be implemented by all backends.
    fn clear_region(&mut self, clear_type: ClearType) -> Result<(), Error> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(Error::new(
                ErrorKind::Other,
                format!("clear_type [{clear_type:?}] not supported with this backend"),
            )),
        }
    }

    fn size(&self) -> Result<Rect, Error>;

    fn flush(&mut self) -> Result<(), Error>;
}

impl<Out> Backend for Termip<Out>
where
    Out: RawOs + Write,
{
    // Can be implemented
    fn draw<'a, I>(&mut self, _content: I) -> Result<(), Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Error> {
        hide_cursor(&mut self.out)
    }

    fn show_cursor(&mut self) -> Result<(), Error> {
        show_cursor(&mut self.out)
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), Error> {
        get_cursor_position(output, input)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Error> {
        move_cursor(&mut self.out, y, x)
    }

    fn clear(&mut self) -> Result<(), Error> {
        erase_entire_screen(&mut self.out)
    }

    fn size(&self) -> Result<Rect, Error> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.out.flush()
    }
}

fn main() -> Result<(), Error> {
    Ok(())
}
