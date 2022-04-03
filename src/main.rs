use anyhow::*;

use log::Level;

use std::io::Write;

#[macro_use]
extern crate log;

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, record| {
            let color = match record.level() {
                Level::Trace => env_logger::fmt::Color::Rgb(35, 175, 200),
                Level::Debug => env_logger::fmt::Color::Rgb(35, 200, 150),
                Level::Info => env_logger::fmt::Color::Rgb(200, 200, 35),
                Level::Warn => env_logger::fmt::Color::Rgb(200, 125, 35),
                Level::Error => env_logger::fmt::Color::Rgb(200, 35, 35),
            };
            let mut style = buf.style();
            style.set_color(color);

            write!(buf, "[{:<5}] ", style.value(record.level()))?;

            style.set_color(env_logger::fmt::Color::Rgb(184, 184, 184));

            write!(
                buf,
                "{} {}{}{}{}",
                style.value(buf.timestamp()),
                style.value(record.module_path().unwrap_or("-")),
                style.value("::"),
                style.value(record.file().unwrap_or("-")),
                style.value("@")
            )?;

            if let Some(line) = record.line() {
                write!(buf, "{}{}", style.value(line), style.value(": "))?;
            } else {
                write!(buf, "{}", style.value("-: "))?;
            }

            writeln!(buf, "{}", record.args())
        })
        .parse_env(env_logger::Env::default())
        .init();

    trace!("Hello, world!");
    debug!("Hello, world!");
    info!("Hello, world!");
    warn!("Hello, world!");
    error!("Hello, world!");

    let args: Vec<_> = std::env::args().collect();

    let first_arg = args.get(1).ok_or(anyhow!("first argument must be the input directory"))?;

    info!("input directory is {}", first_arg);

    let second_arg = args.get(2).ok_or(anyhow!("second argument must be the output directory"))?;

    Ok(())
}
