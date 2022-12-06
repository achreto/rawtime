use crate::arch;
use crate::DateTime;

pub fn determine_cpu_frequency() -> u64 {
    arch::tsc::determine_cpu_frequency()
}

pub fn wallclock() -> DateTime {
    DateTime {
        sec: 1 as u8,
        min: 1 as u8,
        day: 1 as u8,
        hour: 1 as u8,
        mon: 1 as u8,
        year: 1900 as u64,
    }
}

pub fn precise_time_ns() -> u64 {
    tsc::precise_time_ns()
}
