use std::ffi::{c_char, c_int, c_uint};
use std::fmt;

const NOTIFY_STATUS_OK: c_uint = 0;

unsafe extern "C" {
    fn notify_register_check(name: *const c_char, out_token: *mut c_int) -> c_uint;
    fn notify_get_state(token: c_int, state: *mut u64) -> c_uint;
    fn notify_cancel(token: c_int) -> c_uint;
}

// values from OSThermalNotification.h
// https://github.com/tripleCC/Laboratory/blob/a7d1192f25d718e3b01a015ca35bfcef4419e883/AppleSources/Libc-1272.250.1/include/libkern/OSThermalNotification.h#L44-L48
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalPressure {
    Nominal,
    Moderate,
    Heavy,
    Trapping,
    Sleeping,
    Unknown(u64),
}

impl ThermalPressure {
    fn from_state(state: u64) -> Self {
        match state {
            0 => Self::Nominal,
            1 => Self::Moderate,
            2 => Self::Heavy,
            3 => Self::Trapping,
            4 => Self::Sleeping,
            n => Self::Unknown(n),
        }
    }

    pub fn level(self) -> u64 {
        match self {
            Self::Nominal => 0,
            Self::Moderate => 1,
            Self::Heavy => 2,
            Self::Trapping => 3,
            Self::Sleeping => 4,
            Self::Unknown(n) => n,
        }
    }

    pub fn is_throttled(self) -> bool {
        !matches!(self, Self::Nominal)
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Nominal => "No thermal pressure",
            Self::Moderate => "System may reduce performance",
            Self::Heavy => "System is actively throttling",
            Self::Trapping => "Critical thermal pressure, system is severely throttled",
            Self::Sleeping => "Extreme thermal pressure, system may sleep to cool down",
            Self::Unknown(_) => "Unknown thermal pressure level",
        }
    }
}

impl fmt::Display for ThermalPressure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nominal => write!(f, "Nominal"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Heavy => write!(f, "Heavy"),
            Self::Trapping => write!(f, "Trapping"),
            Self::Sleeping => write!(f, "Sleeping"),
            Self::Unknown(n) => write!(f, "Unknown({n})"),
        }
    }
}

pub struct ThermalMonitor {
    token: c_int,
}

impl ThermalMonitor {
    pub fn new() -> Result<Self, String> {
        let name = c"com.apple.system.thermalpressurelevel";
        let mut token: c_int = 0;
        let status = unsafe { notify_register_check(name.as_ptr(), &mut token) };
        if status != NOTIFY_STATUS_OK {
            return Err(format!(
                "failed to register thermal notification (status: {status})"
            ));
        }
        Ok(Self { token })
    }

    pub fn read(&self) -> Result<ThermalPressure, String> {
        let mut state: u64 = 0;
        let status = unsafe { notify_get_state(self.token, &mut state) };
        if status != NOTIFY_STATUS_OK {
            return Err(format!("failed to read thermal state (status: {status})"));
        }
        Ok(ThermalPressure::from_state(state))
    }
}

impl Drop for ThermalMonitor {
    fn drop(&mut self) {
        unsafe { notify_cancel(self.token) };
    }
}
