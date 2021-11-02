use std::io::Result;

pub use windows::Win32::System::Console::CONSOLE_MODE;

use windows::Win32::System::Console::{
    GetConsoleMode, SetConsoleMode, DISABLE_NEWLINE_AUTO_RETURN, ENABLE_ECHO_INPUT,
    ENABLE_INSERT_MODE, ENABLE_LINE_INPUT, ENABLE_LVB_GRID_WORLDWIDE, ENABLE_MOUSE_INPUT,
    ENABLE_PROCESSED_INPUT, ENABLE_PROCESSED_OUTPUT, ENABLE_QUICK_EDIT_MODE,
    ENABLE_VIRTUAL_TERMINAL_INPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING, ENABLE_WINDOW_INPUT,
    ENABLE_WRAP_AT_EOL_OUTPUT,
};

use super::{result, Handle, HandleType};

/// A wrapper around a screen buffer, focusing on calls to get and set the console mode.
///
/// This wraps [`SetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/setconsolemode)
/// and [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode).
#[derive(Debug, Clone)]
pub struct ConsoleMode {
    // the handle used for the functions of this type.
    handle: Handle,
}

impl ConsoleMode {
    /// Create a new `ConsoleMode` instance.
    ///
    /// This will use the standard output as its handle.
    /// When you explicitly want to specify the handle used for the function calls use `ConsoleMode::from(handle)` instead.
    pub fn new() -> Result<ConsoleMode> {
        Ok(ConsoleMode {
            handle: Handle::new(HandleType::OutputHandle)?,
        })
    }

    /// Set the console mode to the given console mode.
    ///
    /// This function sets the `dwMode`.
    ///
    /// This wraps
    /// [`SetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/setconsolemode).
    pub fn set_mode(&self, console_mode: CONSOLE_MODE) -> Result<()> {
        result(unsafe { SetConsoleMode(*self.handle, console_mode) })
    }

    /// Get the console mode.
    ///
    /// This function returns the `lpMode`.
    ///
    /// This wraps
    /// [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode).
    pub fn mode(&self) -> Result<CONSOLE_MODE> {
        let mut console_mode = CONSOLE_MODE::default();
        result(unsafe { GetConsoleMode(*self.handle, &mut console_mode) })?;
        Ok(console_mode)
    }

    /// Console mode constants.
    ///
    /// This group of constants can be used when the console handle refers to an input handle.
    ///
    /// See [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode) for reference.
    pub const ENABLE_ECHO_INPUT: CONSOLE_MODE = ENABLE_ECHO_INPUT;
    pub const ENABLE_INSERT_MODE: CONSOLE_MODE = ENABLE_INSERT_MODE;
    pub const ENABLE_LINE_INPUT: CONSOLE_MODE = ENABLE_LINE_INPUT;
    pub const ENABLE_MOUSE_INPUT: CONSOLE_MODE = ENABLE_MOUSE_INPUT;
    pub const ENABLE_PROCESSED_INPUT: CONSOLE_MODE = ENABLE_PROCESSED_INPUT;
    pub const ENABLE_QUICK_EDIT_MODE: CONSOLE_MODE = ENABLE_QUICK_EDIT_MODE;
    pub const ENABLE_WINDOW_INPUT: CONSOLE_MODE = ENABLE_WINDOW_INPUT;
    pub const ENABLE_VIRTUAL_TERMINAL_INPUT: CONSOLE_MODE = ENABLE_VIRTUAL_TERMINAL_INPUT;

    /// Console mode constants.
    ///
    /// This group of constants can be used when the console handle refers to a screen buffer handle.
    ///
    /// See [`GetConsoleMode`](https://docs.microsoft.com/en-us/windows/console/getconsolemode) for reference.
    pub const ENABLE_PROCESSED_OUTPUT: CONSOLE_MODE = ENABLE_PROCESSED_OUTPUT;
    pub const ENABLE_WRAP_AT_EOL_OUTPUT: CONSOLE_MODE = ENABLE_WRAP_AT_EOL_OUTPUT;
    pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: CONSOLE_MODE = ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    pub const DISABLE_NEWLINE_AUTO_RETURN: CONSOLE_MODE = DISABLE_NEWLINE_AUTO_RETURN;
    pub const ENABLE_LVB_GRID_WORLDWIDE: CONSOLE_MODE = ENABLE_LVB_GRID_WORLDWIDE;
}

impl From<Handle> for ConsoleMode {
    fn from(handle: Handle) -> Self {
        ConsoleMode { handle }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsoleMode;

    // TODO - Test is ignored, because it's failing on Travis CI
    #[test]
    #[ignore]
    fn test_set_get_mode() {
        let mode = ConsoleMode::new().unwrap();

        let original_mode = mode.mode().unwrap();

        mode.set_mode(ConsoleMode::ENABLE_ECHO_INPUT).unwrap();
        let console_mode = mode.mode().unwrap();
        assert_eq!(
            console_mode & ConsoleMode::ENABLE_ECHO_INPUT,
            mode.mode().unwrap()
        );

        mode.set_mode(original_mode).unwrap();
    }
}
