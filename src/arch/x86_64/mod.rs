mod rtc;
pub mod tsc;

use crate::DateTime;

pub(crate) fn determine_cpu_frequency() -> u64 {
    const MHZ_TO_HZ: u64 = 1000000;
    const KHZ_TO_HZ: u64 = 1000;
    let cpuid = x86::cpuid::CpuId::new();

    // Use info from hypervisor if available:
    if let Some(hv) = cpuid.get_hypervisor_info() {
        if let Some(tsc_khz) = hv.tsc_frequency() {
            return tsc_khz as u64 * KHZ_TO_HZ;
        }
    }

    // Use CpuId info if available:
    if let Some(tinfo) = cpuid.get_tsc_info() {
        if let Some(freq) = tinfo.tsc_frequency() {
            return freq;
        } else {
            if tinfo.numerator() != 0 && tinfo.denominator() != 0 {
                // Approximate with the processor frequency:
                if let Some(pinfo) = cpuid.get_processor_frequency_info() {
                    let cpu_base_freq_hz = pinfo.processor_base_frequency() as u64 * MHZ_TO_HZ;
                    let crystal_hz =
                        cpu_base_freq_hz * tinfo.denominator() as u64 / tinfo.numerator() as u64;
                    return crystal_hz * tinfo.numerator() as u64 / tinfo.denominator() as u64;
                }
            }
        }
    }

    3000 * MHZ_TO_HZ
}

pub fn wallclock() -> DateTime {
    unsafe { rtc::now() }
}

pub fn precise_time_ns() -> u64 {
    tsc::precise_time_ns()
}
