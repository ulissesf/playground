mod msr;

use std::thread;
use std::time::Duration;

use anyhow::Result;
use msr::{Msr, MSR_IA32_TEMPERATURE_TARGET, MSR_IA32_PACKAGE_THERM_STATUS};

const TJMAX_DEFAULT: u32 = 100;

// Read TjMax from MSR_IA32_TEMPERATURE_TARGET bits [23:16].
// Falls back to TJMAX_DEFAULT if the MSR is unavailable or returns zero.
fn get_tjmax(msr: &Msr) -> u32 {
    match msr.read(MSR_IA32_TEMPERATURE_TARGET) {
        Ok(val) => {
            let tcc_default = ((val >> 16) & 0xFF) as u32;
            if tcc_default != 0 {
                tcc_default
            } else {
                eprintln!("MSR_IA32_TEMPERATURE_TARGET returned 0, falling back to {TJMAX_DEFAULT} C");
                TJMAX_DEFAULT
            }
        }
        Err(e) => {
            eprintln!("Cannot read MSR_IA32_TEMPERATURE_TARGET: {e}, falling back to {TJMAX_DEFAULT} C");
            TJMAX_DEFAULT
        }
    }
}

// Read package temperature: tjmax minus the DTS readout from bits [22:16]
// of MSR_IA32_PACKAGE_THERM_STATUS.
fn get_pkg_temp(msr: &Msr, tjmax: u32) -> Result<f64> {
    let val = msr.read(MSR_IA32_PACKAGE_THERM_STATUS)?;
    println!("val = {:?}", val);
    let dts = ((val >> 16) & 0xFF) as u32;
    Ok(tjmax.saturating_sub(dts) as f64)
}

fn main() -> Result<()> {
    if !Msr::is_capable() {
        eprintln!("MSR access unavailable (need root and msr kernel module).");
        std::process::exit(1);
    }

    // Use CPU 0 — package thermal status is a package-level MSR, any CPU
    // in the package works.
    let msr = Msr::from(0)?;
    let tjmax = get_tjmax(&msr);
    println!("TjMax: {tjmax} C");

    loop {
        match get_pkg_temp(&msr, tjmax) {
            Ok(temp) => println!("Package temp: {:?} C", temp),
            Err(e) => eprintln!("Error reading package temp: {e}"),
        }
        thread::sleep(Duration::from_millis(1500));
    }
}
