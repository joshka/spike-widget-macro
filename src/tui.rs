use std::io::{self, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::prelude::*;

pub fn init() -> io::Result<(Terminal<impl Backend>, TuiGuard)> {
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(1),
        },
    )?;
    enable_raw_mode()?;
    Ok((terminal, TuiGuard))
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}
pub struct TuiGuard;

impl Drop for TuiGuard {
    fn drop(&mut self) {
        restore().unwrap();
    }
}
