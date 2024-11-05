use anyhow::Result;
use std::path::Path;
use std::thread;
use std::time;
use std::mem;
use std::fs;

use libc;

mod perf_event;
use perf_event::{perf_event_attr, PERF_SAMPLE_IDENTIFIER, PerfEvent};


fn main() -> Result<()> {
    let perf_dir = Path::new("/sys/bus/event_source/devices/power/events");
    let cpu: i32 = unsafe { libc::sched_getcpu() };

    let scale: f64 = fs::read_to_string(perf_dir.join("energy-gpu.scale"))?
        .trim().parse()?;
    let unit = fs::read_to_string(perf_dir.join("energy-gpu.unit"))?.trim();
    let rtype: u32 = fs::read_to_string(
        Path::new("/sys/bus/event_source/devices/power/type"))?
        .trim().parse()?;

    let cstr = fs::read_to_string(perf_dir.join("energy-gpu"))?;
    let cfg_str = cstr.trim();
    let cfg: Vec<_> = cfg_str.split(',').map(|it| it.trim()).collect();
    let mut config: Option<u64> = None;
    let mut umask: u64 = 0;
    for c in cfg.iter() {
        let kv: Vec<_> = c.split('=').map(|it| it.trim()).collect();
        if kv[0].starts_with("event") {
            config = Some(u64::from_str_radix(kv[1].trim_start_matches("0x"), 16)?);
        } else if kv[0].starts_with("umask") {
            umask = kv[1].parse()?;
        } else {
            panic!("Unknwon key {:?} in perf config file", kv[0]);
        }
    }
    if config.is_none() {
        panic!("No config info in perf config file");
    }
    let config = (umask << 8) | config.unwrap();

    let mut pf_attr = perf_event_attr::new();

    pf_attr.type_ = rtype;
    pf_attr.size = mem::size_of::<perf_event_attr>() as u32;
    pf_attr.config = config;
    pf_attr.sample_type = PERF_SAMPLE_IDENTIFIER;
    pf_attr.read_format = 0;

    let pfevt = PerfEvent::open(&pf_attr, -1, cpu, 0)?;
    let onesec = time::Duration::from_secs(5);

    let mut old_val = pfevt.read(1)?[0];
    let mut old_time = time::Instant::now();
    loop {
        thread::sleep(onesec);

        let new_val = pfevt.read(1)?[0];
        let delta_val = new_val - old_val;
        let delta_time = old_time.elapsed().as_secs_f64();

        println!("{:.1?} W", (delta_val as f64 * scale) / delta_time);

        old_val = new_val;
        old_time = time::Instant::now();
    }

    Ok(())
}
