use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::layout::Viewport;
use crate::render::{render_markdown, RenderOptions};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum RenderMode {
    Auto,
    Always,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ColorMode {
    Auto,
    Always,
    Never,
}

#[derive(Debug, Parser)]
#[command(name = "mdr")]
#[command(about = "Render Markdown for comfortable terminal reading")]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    #[arg(long, value_name = "COLUMNS")]
    pub width: Option<usize>,

    #[arg(long, value_enum, default_value_t = RenderMode::Auto)]
    pub render: RenderMode,

    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    pub color: ColorMode,
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    run_with(
        args,
        atty::is(atty::Stream::Stdout),
        atty::is(atty::Stream::Stdin),
    )
}

pub fn run_with(args: Args, stdout_is_tty: bool, _stdin_is_tty: bool) -> Result<()> {
    let source = load_input(args.file.as_ref())?;
    let should_render = args.render == RenderMode::Always || stdout_is_tty;

    let mut stdout = io::stdout().lock();
    if should_render {
        let color = match args.color {
            ColorMode::Always => true,
            ColorMode::Never => false,
            ColorMode::Auto => stdout_is_tty,
        };
        let viewport = Viewport::resolve(args.width);
        let output = render_markdown(&source, RenderOptions { viewport, color });
        stdout.write_all(output.as_bytes())?;
    } else {
        stdout.write_all(source.as_bytes())?;
    }
    Ok(())
}

pub fn load_input(file: Option<&PathBuf>) -> Result<String> {
    if let Some(path) = file {
        return fs::read_to_string(path)
            .with_context(|| format!("failed to read input file '{}'", path.display()));
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("failed to read Markdown from stdin")?;
    Ok(input)
}
