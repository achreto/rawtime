use crate::arch;
use crate::DateTime;
use libc;

pub fn wallclock() -> DateTime {
    unsafe {
        let mut t: libc::time_t = 0;
        libc::time(&mut t);
        let ltime: *mut libc::tm = libc::localtime(&t);

        DateTime {
            sec: (*ltime).tm_sec as u8,
            min: (*ltime).tm_min as u8,
            day: (*ltime).tm_mday as u8,
            hour: (*ltime).tm_hour as u8,
            mon: ((*ltime).tm_mon + 1) as u8,
            year: ((*ltime).tm_year + 1900) as u64,
        }
    }
}

pub fn precise_time_ns() -> u64 {
    arch::tsc::precise_time_ns()
}
