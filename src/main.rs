mod bar_display;
mod mode;
mod term;
mod thermal;

use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use clap::Parser;
use color_print::ceprintln;

use crate::mode::Mode;
use crate::thermal::{ThermalMonitor, ThermalPressure};

const CHECK_INTERVAL: u64 = 5;

/// macOS thermal throttle monitor
#[derive(Parser)]
#[command(name = "toast")]
struct Cli {
    /// Continuously monitor thermal state
    #[arg(short, long)]
    watch: bool,

    /// Show a vertical bar chart of history (requires --watch)
    #[arg(short, long)]
    bar: bool,
}

impl Cli {
    fn run(&self) -> Result<ExitCode, String> {
        let monitor = ThermalMonitor::new()?;
        if self.watch {
            let mode = Mode::try_new(self.bar)?;
            Self::read_loop(&monitor, mode);
        } else {
            Self::read_once(&monitor)
        }
    }

    fn read_once(monitor: &ThermalMonitor) -> Result<ExitCode, String> {
        let pressure = monitor.read()?;
        println!("Thermal pressure: {}", term::colored_label(pressure));
        println!("{}", pressure.description());
        match pressure.is_throttled() {
            true => Ok(ExitCode::from(1)),
            false => Ok(ExitCode::SUCCESS),
        }
    }

    fn read_loop(monitor: &ThermalMonitor, mut mode: Mode) -> ! {
        ceprintln!("<green>Watching thermal pressure</green> <dim>(Ctrl+C to stop)</dim>");
        let mut last: Option<ThermalPressure> = None;
        loop {
            match monitor.read() {
                Ok(pressure) => {
                    mode.on_reading(pressure);
                    if last != Some(pressure) {
                        mode.on_change(pressure);
                        last = Some(pressure);
                    }
                },
                Err(status) => {
                    eprintln!("error: failed to read thermal state (status: {status})");
                },
            }
            for remaining in (1..=CHECK_INTERVAL).rev() {
                mode.on_tick(&format!("Next check in {remaining}s"));
                thread::sleep(Duration::from_secs(1));
            }
            mode.on_cycle_end();
        }
    }
}

fn main() -> ExitCode {
    Cli::parse().run().unwrap_or_else(|e| {
        eprintln!("error: {e}");
        ExitCode::from(2)
    })
}
