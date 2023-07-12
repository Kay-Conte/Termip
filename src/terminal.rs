use std::io::{Read, Write};

use crate::events::EventBatch;

#[cfg(target_family = "windows")]
pub mod platform {

    use std::{
        io::{Error, ErrorKind, Write},
        os::windows::prelude::AsRawHandle,
    };

    use winapi::um::{
        consoleapi::{GetConsoleMode, ReadConsoleInputW, SetConsoleMode},
        handleapi::INVALID_HANDLE_VALUE,
        wincon::{
            PeekConsoleInputW, SetConsoleCursorPosition, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT,
            ENABLE_PROCESSED_INPUT, ENABLE_PROCESSED_OUTPUT,
        },
        wincontypes::{COORD, INPUT_RECORD},
        winnt::HANDLE,
    };

    use crate::{events::Event, key::Key};

    pub trait RawOs: std::os::windows::io::AsRawHandle {}

    impl<T> RawOs for T where T: std::os::windows::io::AsRawHandle {}

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
    pub fn clear_screen<S>(s: &mut S) -> std::io::Result<()>
    where
        S: Write,
    {
        write!(s, "\x1B[2J\x1B[1;1H")?;

        Ok(())
    }

    pub fn try_read_event<S>(stdin: &mut S) -> std::io::Result<Option<Event>>
    where
        S: AsRawHandle,
    {
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

        /// the stdin of the program you want to set. This has no effect on non stdin files.
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

    pub fn move_cursor<S>(stdin: S, x: i16, y: i16) -> std::io::Result<()>
    where
        S: AsRawHandle,
    {
        let handle = stdin.as_raw_handle() as HANDLE;

        let pos = COORD { X: x, Y: y };

        if unsafe { SetConsoleCursorPosition(handle, pos) } == 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }
}

#[cfg(target_family = "unix")]
pub mod platform {
    use std::{
        ffi::c_int,
        io::{Error, Read, Write},
        os::fd::AsRawFd
    };

    use libc::{
        fcntl, ioctl, poll, pollfd, termios, winsize, FIONREAD, F_GETFL, F_SETFL, O_NONBLOCK,
        POLLIN, TCSAFLUSH, TIOCGWINSZ, nfds_t,
    };



    use crate::{
        events::{Event, EventBatch},
        parse::unix::parse_batch,
    };

    pub trait RawOs: std::os::fd::AsRawFd {}

    impl<T> RawOs for T where T: std::os::fd::AsRawFd {}

    pub fn set_non_blocking_read<S>(read: &mut S) -> std::io::Result<()>
    where
        S: AsRawFd,
    {
        let fd = read.as_raw_fd();

        let mut flags = unsafe { fcntl(fd, F_GETFL, 0) };

        if flags == -1 {
            return Err(Error::last_os_error());
        }

        flags |= O_NONBLOCK;

        if unsafe { fcntl(fd, F_SETFL, flags) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn enable_raw_mode<Input>(input: &mut Input) -> std::io::Result<()>
    where
        Input: AsRawFd,
    {
        let fd = input.as_raw_fd();

        let mut opts: termios = unsafe { std::mem::zeroed() };

        if unsafe { libc::tcgetattr(fd, &mut opts) } == -1 {
            return Err(Error::last_os_error());
        }

        opts.c_lflag &= !(libc::ECHO | libc::ICANON);

        if unsafe { libc::tcsetattr(fd, TCSAFLUSH, &mut opts) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn disable_raw_mode<Input>(input: &mut Input) -> std::io::Result<()>
    where
        Input: AsRawFd,
    {
        let fd = input.as_raw_fd();

        let mut opts: termios = unsafe { std::mem::zeroed() };

        if unsafe { libc::tcgetattr(fd, &mut opts) } == -1 {
            return Err(Error::last_os_error());
        }

        opts.c_lflag |= libc::ECHO | libc::ICANON;

        if unsafe { libc::tcsetattr(fd, TCSAFLUSH, &mut opts) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn enter_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()> where Output: Write {
        write!(output, "\x1b[?1049h")
    }

    pub fn leave_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()> where Output: Write {
        write!(output, "\x1b[?1049l")
    }

    pub fn read_batch<Input>(input: &mut Input) -> std::io::Result<EventBatch>
    where
        Input: AsRawFd + Read,
    {
        let fd = input.as_raw_fd();

        let mut bytes_available: c_int = unsafe { std::mem::zeroed() };

        if unsafe { ioctl(fd, FIONREAD, &mut bytes_available) } == -1 {
            return Err(std::io::Error::last_os_error());
        }

        let mut buf = vec![0; bytes_available as usize];

        input.read_exact(&mut buf)?;

        let batch = parse_batch(buf);

        Ok(batch)
    }

    pub fn read_batch_blocking<Input>(
        input: &mut Input,
        timeout: u32,
    ) -> std::io::Result<EventBatch>
    where
        Input: AsRawFd + Read,
    {
        let fd = input.as_raw_fd();

        let mut pfd = [pollfd {
            fd,
            events: POLLIN,
            revents: 0,
        }];

        if unsafe { poll(pfd.as_mut_ptr() , 1 as nfds_t, timeout as c_int) } == -1 {
            return Err(std::io::Error::last_os_error());
        }

        if pfd[0].revents & POLLIN == 0 {
            return Ok(EventBatch::empty());
        }

        let mut bytes_available = 0;

        if unsafe { ioctl(fd, FIONREAD, &mut bytes_available) } == -1 {
            return Err(std::io::Error::last_os_error());
        }

        let mut buf = vec![0; bytes_available as usize];

        input.read_exact(&mut buf)?;

        let batch = parse_batch(buf);

        Ok(batch)
    }

    pub fn request_cursor_position<Output>(output: &mut Output) -> std::io::Result<()>
    where
        Output: Write,
    {
        write!(output, "\x1b[6n")
    }

    pub fn get_cursor_position<Output, Input>(
        output: &mut Output,
        input: &mut Input,
    ) -> std::io::Result<Option<(u16, u16)>>
    where
        Output: Write,
        Input: AsRawFd + Read,
    {
        request_cursor_position(output)?;

        output.flush()?;

        let start = std::time::Instant::now();

        loop {
            let passed = (std::time::Instant::now() - start).as_millis() as u32;

            if passed > 1000 {
                break;
            }

            let batch = read_batch_blocking(input, 1000 - passed)?;

            for ev in batch.into_iter() {
                if let Event::Cursor(x, y) = ev {
                    return Ok(Some((x, y)));
                }
            }

        }

        return Ok(None)
    }

    pub fn move_cursor<Output>(output: &mut Output, line: u16, column: u16) -> std::io::Result<()>
    where
        Output: Write,
    {
        write!(output, "\x1b[{};{}H", line, column)
    }

    pub fn hide_cursor<Output>(output: &mut Output) -> std::io::Result<()>
    where
        Output: Write,
    {
        write!(output, "\x1b[?25l")
    }

    pub fn show_cursor<Output>(output: &mut Output) -> std::io::Result<()>
    where
        Output: Write,
    {
        write!(output, "\x1b[?25h")
    }

    pub fn get_size<Output>(output: &Output) -> std::io::Result<(u16, u16)>
    where
        Output: AsRawFd,
    {
        let fd = output.as_raw_fd();

        let mut size: winsize = unsafe { std::mem::zeroed() };

        if unsafe { ioctl(fd, TIOCGWINSZ, &mut size) } == -1 {
            return Err(Error::last_os_error());
        }

        Ok((size.ws_row, size.ws_col))
    }

    pub fn erase_entire_screen<S>(s: &mut S) -> std::io::Result<()>
    where
        S: Write,
    {
        write!(s, "\x1b[2J")
    }
}

/// This function enables "raw" mode on all platforms. This disables automatic input to output
/// echoing and line buffering
pub fn enable_raw_mode<Input>(input: &mut Input) -> std::io::Result<()>
where
    Input: platform::RawOs,
{
    platform::enable_raw_mode(input)
}


/// This function enters an alternate view on all platforms.
pub fn enter_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()> where Output: Write {
    platform::enter_alternate_buffer(output)
}

/// This function leaves an alternate view on all platforms.
pub fn leave_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()> where Output: Write {
    platform::leave_alternate_buffer(output)
}

/// This function disables "raw" mode on all platforms. This enables automatic input to output
/// echoing and line buffering
pub fn disable_raw_mode<Input>(input: &mut Input) -> std::io::Result<()>
where
    Input: platform::RawOs,
{
    platform::disable_raw_mode(input)
}

/// This function reads a batch of events from an input. This function is non blocking but will
/// return an empty batch if there are no bytes available.
pub fn read_batch<Input>(input: &mut Input) -> std::io::Result<EventBatch>
where
    Input: platform::RawOs + Read,
{
    platform::read_batch(input)
}


/// This function reads a batch of events from an input. This function is a blocking call and will
/// only return an empty batch on timeout.
pub fn read_batch_blocking<Input>(input: &mut Input, timeout: u32) -> std::io::Result<EventBatch>
where
    Input: platform::RawOs + Read,
{
    platform::read_batch_blocking(input, timeout)
}

/// This function returns the current cursor position. This function requires the application be in
/// raw mode or it will return `None` after 1 second. This function is able to block up to 1
/// second on unix platforms because of the nature of the request. For most applications a call to
/// this function will not be close to timeout, however it is still advised to avoid this function if possible.
pub fn get_cursor_position<Output, Input>(
    output: &mut Output,
    input: &mut Input,
) -> std::io::Result<Option<(u16, u16)>>
where
    Output: Write,
    Input: platform::RawOs + Read,
{
    platform::get_cursor_position(output, input)
}

/// This function moves the cursor to a given position.
pub fn move_cursor<Output>(output: &mut Output, line: u16, column: u16) -> std::io::Result<()>
where
    Output: Write,
{
    platform::move_cursor(output, line, column)
}

/// This function hides the cursor
pub fn hide_cursor<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    platform::hide_cursor(output)
}

/// This function shows the cursor
pub fn show_cursor<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    platform::show_cursor(output)
}

/// This function returns the size of an output in rows and columns
pub fn get_size<Output>(output: &Output) -> std::io::Result<(u16, u16)>
where
    Output: platform::RawOs,
{
    platform::get_size(output)
}

/// This function deletes all contents of a terminal
pub fn erase_entire_screen<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    platform::erase_entire_screen(output)
}
