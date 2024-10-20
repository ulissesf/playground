use std::fs::File;
use std::os::fd::AsRawFd;
use std::mem;
use std::alloc;

use anyhow::Result;
use libc;

mod xe_drm;


fn main() -> Result<()> {
    let f = File::open("/dev/dri/card0")?;
    let fd = f.as_raw_fd();

    let mut dq = xe_drm::drm_xe_device_query {
        extensions: 0,
        size: 0,
        data: 0,
        query: xe_drm::DRM_XE_DEVICE_QUERY_CONFIG,
        reserved: [0, 0],
    };

    let mut res: i32;

    unsafe {
        res = libc::ioctl(fd, xe_drm::DRM_IOCTL_XE_DEVICE_QUERY, &mut dq);
    }
    println!("ioctl({:?}) = {:?}", dq, res);

    println!("sizeof(query_config) = {:?}", mem::size_of::<xe_drm::drm_xe_query_config>());

    unsafe {
        let layout = alloc::Layout::from_size_align(dq.size as usize,
            mem::size_of::<u64>()).unwrap();
        let st = alloc::alloc(layout) as *mut xe_drm::drm_xe_query_config;

        dq.data = st as u64;

        res = libc::ioctl(fd, xe_drm::DRM_IOCTL_XE_DEVICE_QUERY, &mut dq);
        let opts =  (*st).info.as_slice((*st).num_params as usize);
        println!("{:?}", opts);
    }
    println!("ioctl({:?}) = {:?}", dq, res);

    Ok(())
}
