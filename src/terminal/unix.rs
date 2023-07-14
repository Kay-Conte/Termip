use std::{
    ffi::c_int,
    io::{Error, Read, Write},
    os::fd::AsRawFd,
};

use libc::{
    fcntl, ioctl, nfds_t, poll, pollfd, termios, winsize, FIONREAD, F_GETFL, F_SETFL,
    O_NONBLOCK, POLLIN, TCSAFLUSH, TIOCGWINSZ,
};

use crate::{
    events::{
        unix::{parse_batch, parse_event},
        Event, EventBatch,
    },
    style::Color,
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

pub fn enter_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    write!(output, "\x1b[?1049h")
}

pub fn leave_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    write!(output, "\x1b[?1049l")
}

pub fn read_single<Input>(input: &mut Input) -> std::io::Result<Option<Event>>
where
    Input: Read,
{
    let mut iter = input.bytes().filter_map(|i| i.ok());

    Ok(parse_event(&mut iter))
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

    if unsafe { poll(pfd.as_mut_ptr(), 1 as nfds_t, timeout as c_int) } == -1 {
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

    return Ok(None);
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
