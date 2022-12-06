use crate::arch;
use crate::DateTime;

pub fn determine_cpu_frequency() -> u64 {
    arch::tsc::determine_cpu_frequency()
}

pub fn wallclock() -> DateTime {
    arch::rtc::now()
}

pub fn precise_time_ns() -> u64 {
    arch::tsc::precise_time_ns()
}
