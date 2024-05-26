use cliproc::{Cli, ExitCode};
use ohm::Ohm;
use std::env;

fn main() -> ExitCode {
    Cli::default().parse(env::args()).go::<Ohm>()
}
