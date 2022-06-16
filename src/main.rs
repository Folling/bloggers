#![feature(register_tool)]
#![feature(iter_intersperse)]
#![feature(main_separator_str)]
#![feature(generic_const_exprs)]
#![feature(const_fmt_arguments_new)]
#![feature(let_chains)]
#![feature(stmt_expr_attributes)]
#![feature(is_some_with)]
#![feature(result_option_inspect)]
#![register_tool(rust_analyzer)]

mod generation;
mod markdown;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Ok, Result};

use crate::markdown::lexer::Lexer;
use crate::markdown::parser::Parser;
use log::{debug, info, Level};

fn sanitise_path<P: AsRef<Path>>(name: &'static str, path: P, dir: bool, must_exist: bool) -> Result<(bool, PathBuf)> {
    let path = path.as_ref();

    info!("sanitising {}", path.display());

    let exists = match path.metadata() {
        Result::Ok(data) => {
            if dir {
                if !data.is_dir() {
                    bail!("{} isn't a directory", name);
                }
            } else if data.is_dir() {
                bail!("{} is a directory", name);
            } else {
                // all is well
            }

            true
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                if must_exist {
                    bail!("{} doesn't exist", name);
                }

                false
            } else {
                bail!("unable to verify whether {} exists", name);
            }
        }
    };

    info!("{} resolves to {}", name, path.display());

    Ok((exists, path.to_path_buf()))
}

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
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

    let args: Vec<_> = std::env::args().collect();

    let first_arg = args
        .get(1)
        .ok_or_else(|| anyhow!("first argument must be the input directory's path"))?;
    let (_, input_path) = sanitise_path("input path", first_arg, true, true)?;

    let second_arg = args
        .get(2)
        .ok_or_else(|| anyhow!("second argument must be the output directory's path"))?;
    let (output_exists, output_path) = sanitise_path("output path", second_arg, true, false)?;

    // recreate output directory from scratch
    if output_exists {
        info!("output path exists, removing");
        for file in std::fs::read_dir(&output_path)? {
            match file?.path() {
                v if v.is_dir() => {
                    std::fs::remove_dir_all(v)?;
                }
                v => {
                    std::fs::remove_file(v)?;
                }
            }
        }
    } else {
        info!("creating output directory");
        std::fs::create_dir_all(&output_path)?;
    }

    let (_, template) = sanitise_path("html template path", input_path.join("template.html"), false, true)?;

    let content = std::fs::read_to_string(template)?;

    let content_needle = "<div id=\"main_container\">";

    // the following isn't optimal, we're searching through the string twice instead of once
    // there doesn't appear to be a standard function that covers this use case and writing up a custom function
    // doesn't seem worth the effort

    let content_idx = content
        .rfind(content_needle)
        .and_then(|v| v.checked_add(content_needle.len()))
        .ok_or_else(|| anyhow!("template.html doesn't contain content needle or integer overflow occurred"))?;

    info!("found content insertion point at idx {}", content_idx);

    // since we're using find we are guaranteed to get the proper byte index
    #[allow(clippy::indexing_slicing, clippy::string_slice)]
    let first = &content[0..content_idx];
    #[allow(clippy::indexing_slicing, clippy::string_slice)]
    let last = &content[content_idx..];

    let mut files = Vec::with_capacity(1024);

    // recurse into all directories
    files.extend(std::fs::read_dir(&input_path)?.filter(|v| v.is_ok_and(|v| !v.path().ends_with("template.html"))));

    while let Some(file) = files.pop() {
        let path = file?.path();

        debug!("generating content for {}", path.display());

        let new_path = path.strip_prefix(&input_path)?;

        let mut output_new_path = output_path.join(new_path);

        if path.is_dir() {
            if let Some(".sass-cache" | ".git") = path.file_name().and_then(std::ffi::OsStr::to_str) {
                debug!("ignoring .sass-cache & .git");
            } else {
                debug!(
                    "path is directory, creating dir {} and appending files to stack",
                    output_new_path.display()
                );
                // no need to call create_dir_all here as all previous dirs are guaranteed to have been created beforehand
                #[allow(clippy::create_dir)]
                std::fs::create_dir(output_new_path)?;
                files.extend(std::fs::read_dir(path)?);
            }
        } else {
            debug!("path is a file");

            let extension = path.extension().and_then(std::ffi::OsStr::to_str);

            match extension {
                Some("bmd") => {
                    debug!("found bmd file, parsing and generating HTML");
                    let content = std::fs::read_to_string(&path)?;
                    output_new_path.set_file_name("index.html");
                    let mut file = std::fs::File::create(output_new_path)?;

                    let mut parser = Parser::new(content);

                    write!(&mut file, "{}{}{}", first, parser.parse()?, last)?;
                }
                // generate css files for scss files
                Some("scss") => {
                    debug!("encountered scss file");

                    // ignore partials
                    if path
                        .file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .is_some_and(|v| v.starts_with('_'))
                    {
                        debug!("scss file is a partial, ignoring");
                    } else {
                        debug!("path is scss, executing sass to generate css file");

                        let output = Command::new("sass")
                            .args(
                                path.to_str()
                                    .ok_or_else(|| anyhow!("unable to convert path {} to str", path.display())),
                            )
                            .output()?;

                        if !output.status.success() {
                            bail!(
                                "unable to generate sass content for {}. Message: {}",
                                path.display(),
                                String::from_utf8(output.stderr)?
                            )
                        }

                        std::fs::write(output_new_path.with_extension("css"), String::from_utf8(output.stdout)?)?;
                    }
                }
                v => {
                    #[allow(clippy::option_if_let_else)]
                    if let Some(v) = v {
                        debug!("extension {} is unknown, copying file to {}", v, output_new_path.display());
                    } else {
                        debug!("no extension exists, copying file to {}", output_new_path.display());
                    }

                    std::fs::copy(path, output_new_path)?;
                }
            }
        }
    }

    Ok(())
}
