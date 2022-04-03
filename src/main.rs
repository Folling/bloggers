use anyhow::*;
use simplelog::*;

#[macro_use]
extern crate log;
extern crate simplelog;

fn main() -> Result<()> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new().clear_filter_ignore().build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    info!("Hello, world!");

    Ok(())
}
