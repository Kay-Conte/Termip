use std::io::{Read, Write};

use crate::events::{Event, EventBatch};

use self::platform::RawOs;

#[cfg(target_family = "windows")]
pub mod windows;

#[cfg(target_family = "windows")]
pub use windows as platform;

#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "unix")]
pub use unix as platform;

/// This function enables "raw" mode on all platforms. This disables automatic input to output
/// echoing and line buffering
pub fn enable_raw_mode<Input>(input: &mut Input) -> std::io::Result<()>
where
    Input: platform::RawOs,
{
    platform::enable_raw_mode(input)
}

/// This function enters an alternate view on all platforms.
pub fn enter_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
    platform::enter_alternate_buffer(output)
}

/// This function leaves an alternate view on all platforms.
pub fn leave_alternate_buffer<Output>(output: &mut Output) -> std::io::Result<()>
where
    Output: Write,
{
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

// This function returns a single event from an input. This function is both blocking and is
// capable of deadlocking in some edgecases on unix. Additionally, this function makes individual
// sys calls to read each byte and can be slow on some platforms.
//
// This function will return `None` on cases where the input is for some reason not blocking by
// default. These cases are not handled but should rarely come up without intention.
//
// If performance and stability is
// important, see `read_batch` or `read_batch_blocking`
pub fn read_single<Input>(input: &mut Input) -> std::io::Result<Option<Event>>
where
    Input: RawOs + Read,
{
    platform::read_single(input)
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
