use libc;
use self::libc::{STDOUT_FILENO, TIOCGWINSZ, c_ushort, ioctl};
pub use self::libc::termios as Termios;
use std::io;


/// A representation of the size of the current terminal
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}

/// Gets the current terminal size
pub fn terminal_size() -> (u16,u16) {
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &us) };
    if r == 0 {
        // because crossterm works starts counting at 0 and unix terminal starts at cell 1 you have subtract one to get 0-based results.
        Some((us.cols -1, us.rows -1))
    } else {
        (0,0)
    }
}

/// Get the current cursor position
pub fn pos() -> (u16,u16)
{
    use std::io::{Write,Read};
    use std::io::Error;

    let command = NoncanonicalModeCommand::new();
    command.execute();

    // This code is original written by term_cursor credits to them.
    let mut stdout = std::io::stdout();

    // Write command
    stdout.write(b"\x1B[6n")?;
    stdout.flush()?;

    // Read back result
    let mut buf = [0u8; 2];
    // Expect `ESC[`
    std::io::stdin().read_exact(&mut buf)?;
    if buf[0] != 0x1B || buf[1] as char != '[' {
        return (0,0);
    }

    // Read rows and cols through a ad-hoc integer parsing function
    let read_num = || -> Result<(i32, char), Error> {
        let mut num = 0;
        let mut c;

        loop {
            let mut buf = [0u8; 1];
            std::io::stdin().read_exact(&mut buf)?;
            c = buf[0] as char;
            if let Some(d) = c.to_digit(10) {
                num = if num == 0 { 0 } else { num * 10 };
                num += d as i32;
            } else {
                break;
            }
        }

        Ok((num, c))
    };

    // Read rows and expect `;`
    let (rows, c) = read_num()?;
    if c != ';' {
        return (0,0);
    }

    // Read cols
    let (cols, c) = read_num()?;

    // Expect `R`
    let res = if c == 'R' { Ok((cols, rows)) } else { return (0,0); };

    command.undo();
    res
}

pub fn set_terminal_mode(terminal: &Termios) -> io::Result<()>
{
    extern "C" {
        pub fn tcsetattr(fd: c_int, opt: c_int, termptr: *const Termios) -> c_int;
    }
    cvt(unsafe { tcsetattr(0, 0, termios) }).and(Ok(()))
}

pub fn get_terminal_mode() -> io::Result<Termios>
{
    extern "C" {
        pub fn tcgetattr(fd: c_int, termptr: *mut Termios) -> c_int;
    }
    unsafe {
        let mut termios = mem::zeroed();
        cvt(tcgetattr(0, &mut termios))?;
        Ok(termios)
    }
}