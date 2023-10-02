use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Term {
    pub fn new() -> Result<Self> {
        let terminal = Self::setup().context("setup failed")?;

        Ok(Self { terminal })
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }

    fn setup() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        let mut stdout = io::stdout();
        enable_raw_mode().context("failed to enable raw mode")?;
        execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
        let terminal =
            Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")?;

        Ok(terminal)
    }

    pub fn restore(mut self) -> Result<()> {
        disable_raw_mode().context("failed to disable raw mode")?;
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen)
            .context("unable to switch to main screen")?;
        self.terminal
            .show_cursor()
            .context("unable to show cursor")?;

        Ok(())
    }
}
