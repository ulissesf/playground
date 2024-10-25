#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::fs::File;
use std::os::fd::AsRawFd;
use std::mem;
use std::alloc;

use anyhow::Result;
use libc;


#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_i915_query_item {
    pub query_id: u64,
    pub length: i32,
    pub flags: u32,
    pub data_ptr: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_i915_query {
    pub num_items: u32,
    pub flags: u32,
    pub items_ptr: u64,
}
const I915_MEMORY_CLASS_SYSTEM: u16 = 0;
const I915_MEMORY_CLASS_DEVICE: u16 = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_i915_gem_memory_class_instance {
    pub memory_class: u16,
    pub memory_instance: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_memory_region_info {
    pub region: drm_i915_gem_memory_class_instance,
    pub rsvd0: u32,
    pub probed_size: u64,
    pub unallocated_size: u64,
    pub extra_info: drm_i915_memory_region_info_extra_info,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union drm_i915_memory_region_info_extra_info {
    pub rsvd1: [u64; 8usize],
    pub cpu: drm_i915_memory_region_info_cpu_visible_memory,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_i915_memory_region_info_cpu_visible_memory {
    pub probed_cpu_visible_size: u64,
    pub unallocated_cpu_visible_size: u64,
}
#[repr(C)]
pub struct drm_i915_query_memory_regions {
    pub num_regions: u32,
    pub rsvd: [u32; 3usize],
    pub regions: __IncompleteArrayField<drm_i915_memory_region_info>,
}

const DRM_IOCTL_I915_QUERY: u64 = 3222299769;
const DRM_I915_QUERY_MEMORY_REGIONS: u64 = 4;

fn main() -> Result<()> {
    let f = File::open("/dev/dri/card2")?;
    let fd = f.as_raw_fd();

    let mut dqi = drm_i915_query_item {
        query_id: DRM_I915_QUERY_MEMORY_REGIONS,
        length: 0,
        flags: 0,
        data_ptr: 0,
    };
    let dqi_ptr: *mut drm_i915_query_item = &mut dqi;

    let mut dq = drm_i915_query {
        num_items: 1,
        flags: 0,
        items_ptr: dqi_ptr as u64,
    };

    let mut res: i32;

    unsafe {
        res = libc::ioctl(fd, DRM_IOCTL_I915_QUERY, &mut dq);
    }
    println!("ioctl({:?}) = {:?}", dq, res);

    unsafe {
        let layout = alloc::Layout::from_size_align(dqi.length as usize,
            mem::align_of::<u64>()).unwrap();
        let mrg = alloc::alloc_zeroed(layout) as *mut drm_i915_query_memory_regions;

        dqi.data_ptr = mrg as u64;

        res = libc::ioctl(fd, DRM_IOCTL_I915_QUERY, &mut dq);
        println!("ioctl({:?}) = {:?}", dq, res);

        let rgs = (*mrg).regions.as_slice((*mrg).num_regions as usize);
        for it in rgs {
            println!("rg = {:?}, probed_size = {:?}, unallocated_size = {:?}",
                it.region, it.probed_size, it.unallocated_size);
        }
    }

    Ok(())
}
