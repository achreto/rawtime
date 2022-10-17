use crate::DateTime;

pub fn now() -> DateTime {
    DateTime {
        sec: 0 as u8,
        min: 0 as u8,
        day: 1 as u8,
        hour: 0 as u8,
        mon: 1 as u8,
        year: 1900 as u64,
    }
}
