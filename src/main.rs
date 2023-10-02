use anyhow::Result;
use app::App;

mod app;
mod backend;
mod layout;
mod term;

fn main() {
    if let Err(e) = run() {
        eprintln!("{:?}", e);
    }
}

fn run() -> Result<()> {
    App::new()?.run()?;

    Ok(())
}
