mod term;
mod thermal;

use clap::Parser;
use color_print::{ceprintln, cprintln};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use thermal::{ThermalMonitor, ThermalPressure};

const CHECK_INTERVAL: u64 = 5;

/// macOS thermal throttle monitor
#[derive(Parser)]
#[command(name = "heattech")]
struct Cli {
    /// Continuously monitor thermal state
    #[arg(short, long)]
    watch: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let monitor = match ThermalMonitor::new() {
        Ok(m) => m,
        Err(status) => {
            eprintln!("error: failed to register thermal notification (status: {status})");
            return ExitCode::from(2);
        }
    };

    if cli.watch {
        ceprintln!("<green>Watching thermal pressure</green> <dim>(Ctrl+C to stop)</dim>");
        let mut last: Option<ThermalPressure> = None;
        loop {
            match monitor.read() {
                Ok(pressure) => {
                    if last != Some(pressure) {
                        term::clear_line();
                        cprintln!(
                            "{} {} <dim>({})</dim>",
                            term::timestamp(),
                            term::colored_label(pressure),
                            pressure.description()
                        );
                        last = Some(pressure);
                    }
                }
                Err(status) => {
                    eprintln!("error: failed to read thermal state (status: {status})");
                }
            }
            for remaining in (1..=CHECK_INTERVAL).rev() {
                term::status(&format!("Next check in {remaining}s"));
                thread::sleep(Duration::from_secs(1));
            }
            term::clear_line();
        }
    } else {
        match monitor.read() {
            Ok(pressure) => {
                println!("Thermal pressure: {}", term::colored_label(pressure));
                println!("{}", pressure.description());
                if pressure.is_throttled() {
                    return ExitCode::from(1);
                }
                ExitCode::SUCCESS
            }
            Err(status) => {
                eprintln!("error: failed to read thermal state (status: {status})");
                ExitCode::from(2)
            }
        }
    }
}
