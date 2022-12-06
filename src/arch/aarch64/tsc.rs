use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::Readable;

pub fn determine_cpu_frequency() -> u64 {
    panic!("figure out cpu frequence on aarch64")
}

lazy_static! {
    /// TSC Frequency in Hz
    pub static ref TSC_FREQUENCY: u64 = {
        CNTFRQ_EL0.get()
    };

}

#[inline]
pub fn precise_time_ns() -> u64 {
    barrier::isb(barrier::SY);
    CNTPCT_EL0.get()
}

#[inline]
pub fn rdtsc() -> u64 {
    barrier::isb(barrier::SY);
    CNTPCT_EL0.get()
}
