use color_print::cprintln;

use crate::bar_display::BarDisplay;
use crate::term;
use crate::thermal::ThermalPressure;

pub enum Mode {
    Plain,
    Bar(BarDisplay),
}

impl Mode {
    pub fn try_new(bar: bool) -> Result<Self, String> {
        if bar {
            let display =
                BarDisplay::new().map_err(|e| format!("failed to initialize bar display: {e}"))?;
            Ok(Mode::Bar(display))
        } else {
            Ok(Mode::Plain)
        }
    }

    pub fn on_reading(&mut self, pressure: ThermalPressure) {
        if let Self::Bar(display) = self {
            display.push(pressure);
        }
    }

    pub fn on_change(&mut self, pressure: ThermalPressure) {
        match self {
            Self::Plain => {
                term::clear_line();
                cprintln!(
                    "{} {} <dim>({})</dim>",
                    term::timestamp(),
                    term::colored_label(pressure),
                    pressure.description()
                );
            },
            Self::Bar(display) => {
                display.print_status(pressure).ok();
            },
        }
    }

    pub fn on_tick(&mut self, countdown: &str) {
        match self {
            Self::Plain => term::status(countdown),
            Self::Bar(display) => {
                display.draw(countdown).ok();
            },
        }
    }

    pub fn on_cycle_end(&mut self) {
        if let Self::Plain = self {
            term::clear_line();
        }
    }
}
