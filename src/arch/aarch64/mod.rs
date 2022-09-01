extern crate armv8;

use crate::DateTime;

pub(crate) fn determine_cpu_frequency() -> u64 {
    panic!("Not implemented");
}

pub fn wallclock() -> DateTime {
    panic!("Not implemented");
}

pub fn precise_time_ns() -> u64 {
    panic!("Not implemented");
}
