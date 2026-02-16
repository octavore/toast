use std::io::{Write, stdout};

use color_print::cformat;

use crate::thermal::ThermalPressure;

pub fn clear_line() {
    print!("\r\x1b[2K");
    let _ = stdout().flush();
}

pub fn status(msg: &str) {
    let dim = cformat!("<dim>{msg}</>");
    print!("\r\x1b[2K{dim}");
    let _ = stdout().flush();
}

pub fn timestamp() -> String {
    let now = chrono::Local::now().format("%H:%M:%S");
    cformat!("<dim>{now}</>")
}

pub fn colored_label(pressure: ThermalPressure) -> String {
    match pressure {
        ThermalPressure::Nominal => cformat!("<green,bold>Nominal</>"),
        ThermalPressure::Moderate => cformat!("<yellow,bold>Moderate</>"),
        ThermalPressure::Heavy => cformat!("<red,bold>Heavy</>"),
        ThermalPressure::Trapping => cformat!("<red,bold,underline>Trapping</>"),
        ThermalPressure::Sleeping => cformat!("<magenta,bold,underline>Sleeping</>"),
        ThermalPressure::Unknown(n) => cformat!("<dim>Unknown({n})</>"),
    }
}
