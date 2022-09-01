
lazy_static! {
    /// TSC Frequency in Hz
    pub static ref TSC_FREQUENCY: u64 = {
        panic!("not implemented");
        0
    };

}

#[inline]
pub fn precise_time_ns() -> u64 {
    panic!("not implemented");
    0
}
