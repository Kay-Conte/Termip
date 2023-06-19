use std::os::windows::prelude::AsRawHandle;

#[cfg(target_os = "windows")]
mod platform {

    use std::{
        io::{Write, Error, ErrorKind},
        os::windows::prelude::AsRawHandle,
    };

    use winapi::um::{
        consoleapi::{GetConsoleMode, SetConsoleMode, ReadConsoleInputW},
        handleapi::INVALID_HANDLE_VALUE,
        wincon::{ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_PROCESSED_OUTPUT, PeekConsoleInputW, SetConsoleCursorPosition},
        winnt::HANDLE, wincontypes::{INPUT_RECORD, COORD},
    };

    use crate::{events::Event, key::Key};

    pub fn set_non_blocking_read() -> std::io::Result<()> {
        Ok(())
    }

    pub fn enable_raw_mode<S>(s: &mut S) -> std::io::Result<()>
    where
        S: AsRawHandle,
    {
        let raw_handle = s.as_raw_handle() as HANDLE;

        if raw_handle == INVALID_HANDLE_VALUE {
            return Err(Error::new(ErrorKind::Other, "Stdin Handle Invalid"));
        }

        let mut mode: u32 = 0;
        if unsafe { GetConsoleMode(raw_handle as *mut _, &mut mode) } == 0 {
            return Err(Error::last_os_error());
        }

        mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);

        if unsafe { SetConsoleMode(raw_handle as *mut _, mode) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn enter_alternate_view<W>(s: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        write!(s, "\x1B[?1049h")?;

        Ok(())
    }

    pub fn leave_alternative_view<S>(s: &mut S) -> std::io::Result<()>
    where
        S: AsRawHandle,
    {
        let raw_handle = s.as_raw_handle() as HANDLE;

        if unsafe { SetConsoleMode(raw_handle as *mut _, ENABLE_PROCESSED_OUTPUT) == 0 } {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    // Clears the screen and returns the cursor to 0, 0
    pub fn clear_screen<S>(s: &mut S) -> std::io::Result<()> where S: Write {
        write!(s, "\x1B[2J\x1B[1;1H")?;
        Ok(())
    }

    pub fn try_read_event<S>(stdin: &mut S) -> std::io::Result<Option<Event>> where S: AsRawHandle {
        let handle = stdin.as_raw_handle() as HANDLE;

        let mut records: [INPUT_RECORD; 1] = unsafe { std::mem::zeroed() };
        let mut records_read = 0;

        if unsafe {
            PeekConsoleInputW(
                handle,
                records.as_mut_ptr(),
                records.len() as u32,
                &mut records_read,
            )
        } == 0
        {
            return Err(Error::last_os_error());
        }

        if records_read == 0 {
            return Ok(None);
        }

        unsafe {
            ReadConsoleInputW(
                handle,
                records.as_mut_ptr(),
                records.len() as u32,
                &mut records_read,
            );
        }

        match unsafe { records[0].EventType } {
            KEY_EVENT => {
                let key_event = unsafe { records[0].Event.KeyEvent() };

                let key: Key = (*key_event).into();

                Ok(Some(Event::Key(key)))
            }

            _ => Ok(None),
        }
    }

    pub fn move_cursor<S>(stdin: S, x: i16, y: i16) -> std::io::Result<()> where S: AsRawHandle {
        let handle = stdin.as_raw_handle() as HANDLE;

        let pos = COORD { X: x, Y: y };

        if unsafe { SetConsoleCursorPosition(handle, pos) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn try_read_mouse_event(stdin: ()) -> std::io::Result<()> {
        unimplemented!()
    }
}

#[cfg(target_os = "unix")]
mod platform {

    pub fn set_non_blocking_read() -> std::io::Result<()> {
        todo!()
    }

    pub fn enable_raw_mode() -> std::io::Result<()> {
        todo()
    }
}

pub fn enable_raw_mode<S>(s: &mut S) -> std::io::Result<()>
where
    S: AsRawHandle,
{
    platform::enable_raw_mode(s)
}

/// This function is intended for use in pair with `try_read_event`. This function does nothing on windows platforms, but makes the passed in file non-blocking for reads on posix compliance systems. If this function is called, you can be sure that the buffer is non-blocking for any calls to `try_read_event` for any platform
pub fn set_non_blocking_read() -> std::io::Result<()> {
    platform::set_non_blocking_read()
}

#[cfg(test)]
mod tests {

    /// Tests the functionality of `enable_raw_mode`
    #[test]
    fn enable_raw_mode() -> std::io::Result<()> {
        let mut stdin = std::io::stdin();

        super::enable_raw_mode(&mut stdin)?;

        Ok(())
    }
}