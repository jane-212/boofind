use anyhow::Result;
use app::App;

mod app;
mod backend;
mod layout;
mod term;

fn main() -> Result<()> {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
    }

    Ok(())
}

fn run() -> Result<()> {
    App::new()?.run()?;

    Ok(())
}
