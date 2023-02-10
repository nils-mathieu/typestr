//! A simple CLI tool to send keys using a script command.

use std::process::ExitCode;

use clap::Parser;
use keysim::Simulator;
use rand_distr::{Distribution, Normal};

/// A tool to simulate keystrokes.
#[derive(Debug, Parser)]
struct Args {
    /// Defines how unstable the key simulation is.
    #[clap(long, short = 'u', default_value_t = 0.0)]
    unstable_rate: f64,
    /// How many keystrokes should be simulated per second.
    #[clap(long, short = 's', default_value_t = 5.0)]
    speed: f64,
    /// Whether simulation errors should be ignored.
    #[clap(long, short = 'i', action, default_value_t = false)]
    ignore_errors: bool,
    /// The string to be simulated.
    to_send: String,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match simulate(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Executes the user's query.
fn simulate(args: &Args) -> Result<(), keysim::Error> {
    let sim = Simulator::new()?;

    let mut rng = rand::thread_rng();
    let dist = Normal::new(args.speed, args.unstable_rate).unwrap();

    for c in args.to_send.chars() {
        let freq: f64 = dist.sample(&mut rng).abs();
        std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / freq));

        let ret = sim.send_char(c);
        if args.ignore_errors {
            let _ = ret;
        } else {
            ret?;
        }
    }

    Ok(())
}
