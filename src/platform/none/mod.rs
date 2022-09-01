use create::arch;

pub fn wallclock() -> DateTime {
    arch::rtc::now()
}

pub fn precise_time_ns() -> u64 {
    arch::tsc::precise_time_ns()
}
