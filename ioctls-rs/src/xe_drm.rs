#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

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

pub const DRM_XE_DEVICE_QUERY_MEM_REGIONS: u32 = 1;
pub const DRM_XE_DEVICE_QUERY_CONFIG: u32 = 2;
pub const DRM_XE_QUERY_CONFIG_REV_AND_DEVICE_ID: u32 = 0;
pub const DRM_XE_QUERY_CONFIG_FLAGS: u32 = 1;
pub const DRM_XE_QUERY_CONFIG_FLAG_HAS_VRAM: u32 = 1;
pub const DRM_XE_QUERY_CONFIG_MIN_ALIGNMENT: u32 = 2;
pub const DRM_XE_QUERY_CONFIG_VA_BITS: u32 = 3;
pub const DRM_XE_QUERY_CONFIG_MAX_EXEC_QUEUE_PRIORITY: u32 = 4;
pub const DRM_IOCTL_XE_DEVICE_QUERY: u64 = 3223872576;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_xe_device_query {
    pub extensions: u64,
    pub query: u32,
    pub size: u32,
    pub data: u64,
    pub reserved: [u64; 2usize],
}

pub type drm_xe_memory_class = ::std::os::raw::c_uint;
pub const DRM_XE_MEM_REGION_CLASS_SYSMEM: drm_xe_memory_class = 0;
pub const DRM_XE_MEM_REGION_CLASS_VRAM: drm_xe_memory_class = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drm_xe_mem_region {
    pub mem_class: u16,
    pub instance: u16,
    pub min_page_size: u32,
    pub total_size: u64,
    pub used: u64,
    pub cpu_visible_size: u64,
    pub cpu_visible_used: u64,
    pub reserved: [u64; 6usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct drm_xe_query_mem_regions {
    pub num_mem_regions: u32,
    pub pad: u32,
    pub mem_regions: __IncompleteArrayField<drm_xe_mem_region>,
}

#[repr(C)]
#[derive(Debug)]
pub struct drm_xe_query_config {
    pub num_params: u32,
    pub pad: u32,
    pub info: __IncompleteArrayField<u64>,
}
