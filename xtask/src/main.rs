use std::env;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(task) = args.next() else {
        print_help();
        return Ok(());
    };

    match task.as_str() {
        "fmt" => cargo(["fmt", "--all"]),
        "lint" => cargo([
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ]),
        "test" => cargo(["test", "--all-targets", "--all-features"]),
        "bench" => cargo(["bench"]),
        "check" => {
            cargo(["fmt", "--all"])?;
            cargo([
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ])?;
            cargo(["test", "--all-targets", "--all-features"])
        }
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        other => Err(format!(
            "unknown xtask '{other}'\n\nRun `cargo xtask help` for available tasks."
        )),
    }
}

fn cargo<const N: usize>(args: [&str; N]) -> Result<(), String> {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let status = Command::new(cargo)
        .args(args)
        .status()
        .map_err(|error| format!("failed to run cargo: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("cargo command failed with status {status}"))
    }
}

fn print_help() {
    println!(
        "\
Usage: cargo xtask <task>

Tasks:
  fmt      Format the workspace
  lint     Run clippy with warnings denied
  test     Run the full test suite
  bench    Run benchmarks
  check    Run fmt, lint, and test
"
    );
}
