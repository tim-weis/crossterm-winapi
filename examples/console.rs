#[cfg(windows)]
use std::io::Result;

#[cfg(windows)]
use crossterm_winapi::ConsoleMode;

#[cfg(windows)]
fn change_console_mode() -> Result<()> {
    let console_mode = ConsoleMode::new()?;

    // get the current console mode:
    let _mode = console_mode.mode()?;

    // set the console mode (not sure if this is an actual value xp)
    console_mode.set_mode(ConsoleMode::ENABLE_LINE_INPUT | ConsoleMode::ENABLE_WINDOW_INPUT)
}

#[cfg(windows)]
fn main() -> Result<()> {
    change_console_mode()
}

#[cfg(not(windows))]
fn main() {
    println!("This example is for the Windows platform only.");
}
