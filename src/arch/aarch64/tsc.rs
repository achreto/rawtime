use cortex_a::{asm::barrier, registers::*};
use tock_registers::interfaces::Readable;

lazy_static! {
    /// TSC Frequency in Hz
    pub static ref TSC_FREQUENCY: u64 = {
        let freq = CNTFRQ_EL0.get()
    };

}

#[inline]
pub fn precise_time_ns() -> u64 {
    barrier::isb(barrier::SY);
    let cnt = CNTPCT_EL0.get();
}
