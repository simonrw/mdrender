use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum PagerMode {
    Auto,
    Always,
    Never,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputSink {
    RawStdout,
    RenderedStdout,
    RenderedPager,
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

    /// Control whether rendered terminal output is shown through less
    #[arg(long, value_enum, default_value_t = PagerMode::Auto)]
    pub pager: PagerMode,
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
    let sink = output_sink(args.render, args.pager, stdout_is_tty);

    let mut stdout = io::stdout().lock();
    match sink {
        OutputSink::RawStdout => stdout.write_all(source.as_bytes())?,
        OutputSink::RenderedStdout | OutputSink::RenderedPager => {
            let color = match args.color {
                ColorMode::Always => true,
                ColorMode::Never => false,
                ColorMode::Auto => stdout_is_tty,
            };
            let viewport = Viewport::resolve(args.width);
            let output = render_markdown(&source, RenderOptions { viewport, color });
            match sink {
                OutputSink::RenderedStdout => stdout.write_all(output.as_bytes())?,
                OutputSink::RenderedPager => write_to_less(&output)?,
                OutputSink::RawStdout => unreachable!("raw output handled above"),
            }
        }
    }
    Ok(())
}

pub fn output_sink(render: RenderMode, pager: PagerMode, stdout_is_tty: bool) -> OutputSink {
    let should_render = render == RenderMode::Always || stdout_is_tty;
    if !should_render {
        return OutputSink::RawStdout;
    }

    match pager {
        PagerMode::Auto if stdout_is_tty => OutputSink::RenderedPager,
        PagerMode::Always => OutputSink::RenderedPager,
        PagerMode::Auto | PagerMode::Never => OutputSink::RenderedStdout,
    }
}

pub fn pager_command() -> (&'static str, [&'static str; 1]) {
    ("less", ["-R"])
}

fn write_to_less(output: &str) -> Result<()> {
    let (program, args) = pager_command();
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| "failed to launch pager 'less -R'")?;

    {
        let stdin = child.stdin.as_mut().context("failed to open pager stdin")?;
        stdin
            .write_all(output.as_bytes())
            .context("failed to write rendered output to pager")?;
    }

    let status = child.wait().context("failed to wait for pager")?;
    if !status.success() {
        anyhow::bail!("pager 'less -R' exited with status {status}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_sink_preserves_raw_passthrough_for_auto_non_tty() {
        assert_eq!(
            output_sink(RenderMode::Auto, PagerMode::Auto, false),
            OutputSink::RawStdout
        );
        assert_eq!(
            output_sink(RenderMode::Auto, PagerMode::Always, false),
            OutputSink::RawStdout
        );
        assert_eq!(
            output_sink(RenderMode::Auto, PagerMode::Never, false),
            OutputSink::RawStdout
        );
    }

    #[test]
    fn output_sink_pages_auto_rendered_tty_output() {
        assert_eq!(
            output_sink(RenderMode::Auto, PagerMode::Auto, true),
            OutputSink::RenderedPager
        );
        assert_eq!(
            output_sink(RenderMode::Always, PagerMode::Auto, true),
            OutputSink::RenderedPager
        );
    }

    #[test]
    fn output_sink_can_disable_pager_for_rendered_output() {
        assert_eq!(
            output_sink(RenderMode::Auto, PagerMode::Never, true),
            OutputSink::RenderedStdout
        );
        assert_eq!(
            output_sink(RenderMode::Always, PagerMode::Never, false),
            OutputSink::RenderedStdout
        );
    }

    #[test]
    fn output_sink_can_force_pager_for_rendered_output() {
        assert_eq!(
            output_sink(RenderMode::Always, PagerMode::Always, false),
            OutputSink::RenderedPager
        );
        assert_eq!(
            output_sink(RenderMode::Always, PagerMode::Always, true),
            OutputSink::RenderedPager
        );
    }

    #[test]
    fn pager_command_is_hard_coded_to_less() {
        std::env::set_var("PAGER", "bat");

        let (program, args) = pager_command();

        assert_eq!(program, "less");
        assert_eq!(args, ["-R"]);
    }
}
