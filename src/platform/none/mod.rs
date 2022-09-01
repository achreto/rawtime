use crate::arch;
use crate::DateTime;

pub fn wallclock() -> DateTime {
    arch::rtc::now()
}

pub fn precise_time_ns() -> u64 {
    arch::tsc::precise_time_ns()
}
