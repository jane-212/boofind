use anyhow::Result;
use app::App;

mod app;
mod term;
mod layout;
mod backend;

fn main() -> Result<()> {
    App::new()?.run()?;

    Ok(())
}
