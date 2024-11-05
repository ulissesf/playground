#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::mem;
use std::io;

use anyhow::Result;
use libc;


// rust-bindgen 0.69.5 on Linux kernel v6.12 uapi perf_event.h + changes
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}

impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }

    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}

pub const PERF_SAMPLE_IP: u64 = 1;
pub const PERF_SAMPLE_TID: u64 = 2;
pub const PERF_SAMPLE_TIME: u64 = 4;
pub const PERF_SAMPLE_ADDR: u64 = 8;
pub const PERF_SAMPLE_READ: u64 = 16;
pub const PERF_SAMPLE_CALLCHAIN: u64 = 32;
pub const PERF_SAMPLE_ID: u64 = 64;
pub const PERF_SAMPLE_CPU: u64 = 128;
pub const PERF_SAMPLE_PERIOD: u64 = 256;
pub const PERF_SAMPLE_STREAM_ID: u64 = 512;
pub const PERF_SAMPLE_RAW: u64 = 1024;
pub const PERF_SAMPLE_BRANCH_STACK: u64 = 2048;
pub const PERF_SAMPLE_REGS_USER: u64 = 4096;
pub const PERF_SAMPLE_STACK_USER: u64 = 8192;
pub const PERF_SAMPLE_WEIGHT: u64 = 16384;
pub const PERF_SAMPLE_DATA_SRC: u64 = 32768;
pub const PERF_SAMPLE_IDENTIFIER: u64 = 65536;
pub const PERF_SAMPLE_TRANSACTION: u64 = 131072;
pub const PERF_SAMPLE_REGS_INTR: u64 = 262144;
pub const PERF_SAMPLE_PHYS_ADDR: u64 = 524288;
pub const PERF_SAMPLE_AUX: u64 = 1048576;
pub const PERF_SAMPLE_CGROUP: u64 = 2097152;
pub const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 4194304;
pub const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 8388608;
pub const PERF_SAMPLE_WEIGHT_STRUCT: u64 = 16777216;
pub const PERF_SAMPLE_MAX: u64 = 33554432;

pub const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1;
pub const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 2;
pub const PERF_FORMAT_ID: u64 = 4;
pub const PERF_FORMAT_GROUP: u64 = 8;
pub const PERF_FORMAT_LOST: u64 = 16;
pub const PERF_FORMAT_MAX: u64 = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_sample {
    pub sample_period: u64,
    pub sample_freq: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_wakeup {
    pub wakeup_events: u32,
    pub wakeup_watermark: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_config1 {
    pub bp_addr: u64,
    pub kprobe_func: u64,
    pub uprobe_path: u64,
    pub config1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_event_attr_config2 {
    pub bp_len: u64,
    pub kprobe_addr: u64,
    pub probe_offset: u64,
    pub config2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_attr {
    pub type_: u32,
    pub size: u32,
    pub config: u64,
    pub sample: perf_event_attr_sample,
    pub sample_type: u64,
    pub read_format: u64,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
    pub wakeup: perf_event_attr_wakeup,
    pub bp_type: u32,
    pub config1: perf_event_attr_config1,
    pub config2: perf_event_attr_config2,
    pub branch_sample_type: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: i32,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
    pub aux_sample_size: u32,
    pub __reserved_3: u32,
    pub sig_data: u64,
    pub config3: u64,
}

impl perf_event_attr {
    #[inline]
    pub fn disabled(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_disabled(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn inherit(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_inherit(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn pinned(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_pinned(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclusive(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclusive(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_user(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_user(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_kernel(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_kernel(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_hv(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_hv(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_idle(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_idle(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn mmap(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_mmap(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn comm(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_comm(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn freq(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_freq(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn inherit_stat(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_inherit_stat(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn enable_on_exec(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_enable_on_exec(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn task(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_task(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn watermark(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_watermark(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn precise_ip(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 2u8) as u64) }
    }

    #[inline]
    pub fn set_precise_ip(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 2u8, val as u64)
        }
    }

    #[inline]
    pub fn mmap_data(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(17usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_mmap_data(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(17usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn sample_id_all(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(18usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_sample_id_all(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(18usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_host(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(19usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_host(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(19usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_guest(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(20usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_guest(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_callchain_kernel(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(21usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_callchain_kernel(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(21usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn exclude_callchain_user(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(22usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_exclude_callchain_user(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(22usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn mmap2(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(23usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_mmap2(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(23usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn comm_exec(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_comm_exec(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn use_clockid(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(25usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_use_clockid(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(25usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn context_switch(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(26usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_context_switch(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(26usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn write_backward(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(27usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_write_backward(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(27usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn namespaces(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(28usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_namespaces(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(28usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn ksymbol(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_ksymbol(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(29usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn bpf_event(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_bpf_event(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn aux_output(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_aux_output(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn cgroup(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_cgroup(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn text_poke(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(33usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_text_poke(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(33usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn build_id(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(34usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_build_id(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(34usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn inherit_thread(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(35usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_inherit_thread(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(35usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn remove_on_exec(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(36usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_remove_on_exec(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(36usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn sigtrap(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(37usize, 1u8) as u64) }
    }

    #[inline]
    pub fn set_sigtrap(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(37usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn __reserved_1(&self) -> u64 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(38usize, 26u8) as u64) }
    }

    #[inline]
    pub fn set___reserved_1(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(38usize, 26u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        disabled: u64,
        inherit: u64,
        pinned: u64,
        exclusive: u64,
        exclude_user: u64,
        exclude_kernel: u64,
        exclude_hv: u64,
        exclude_idle: u64,
        mmap: u64,
        comm: u64,
        freq: u64,
        inherit_stat: u64,
        enable_on_exec: u64,
        task: u64,
        watermark: u64,
        precise_ip: u64,
        mmap_data: u64,
        sample_id_all: u64,
        exclude_host: u64,
        exclude_guest: u64,
        exclude_callchain_kernel: u64,
        exclude_callchain_user: u64,
        mmap2: u64,
        comm_exec: u64,
        use_clockid: u64,
        context_switch: u64,
        write_backward: u64,
        namespaces: u64,
        ksymbol: u64,
        bpf_event: u64,
        aux_output: u64,
        cgroup: u64,
        text_poke: u64,
        build_id: u64,
        inherit_thread: u64,
        remove_on_exec: u64,
        sigtrap: u64,
        __reserved_1: u64,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let disabled: u64 = unsafe { ::std::mem::transmute(disabled) };
            disabled as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let inherit: u64 = unsafe { ::std::mem::transmute(inherit) };
            inherit as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let pinned: u64 = unsafe { ::std::mem::transmute(pinned) };
            pinned as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let exclusive: u64 = unsafe { ::std::mem::transmute(exclusive) };
            exclusive as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let exclude_user: u64 = unsafe { ::std::mem::transmute(exclude_user) };
            exclude_user as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let exclude_kernel: u64 = unsafe { ::std::mem::transmute(exclude_kernel) };
            exclude_kernel as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let exclude_hv: u64 = unsafe { ::std::mem::transmute(exclude_hv) };
            exclude_hv as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let exclude_idle: u64 = unsafe { ::std::mem::transmute(exclude_idle) };
            exclude_idle as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let mmap: u64 = unsafe { ::std::mem::transmute(mmap) };
            mmap as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let comm: u64 = unsafe { ::std::mem::transmute(comm) };
            comm as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let freq: u64 = unsafe { ::std::mem::transmute(freq) };
            freq as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let inherit_stat: u64 = unsafe { ::std::mem::transmute(inherit_stat) };
            inherit_stat as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let enable_on_exec: u64 = unsafe { ::std::mem::transmute(enable_on_exec) };
            enable_on_exec as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let task: u64 = unsafe { ::std::mem::transmute(task) };
            task as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let watermark: u64 = unsafe { ::std::mem::transmute(watermark) };
            watermark as u64
        });
        __bindgen_bitfield_unit.set(15usize, 2u8, {
            let precise_ip: u64 = unsafe { ::std::mem::transmute(precise_ip) };
            precise_ip as u64
        });
        __bindgen_bitfield_unit.set(17usize, 1u8, {
            let mmap_data: u64 = unsafe { ::std::mem::transmute(mmap_data) };
            mmap_data as u64
        });
        __bindgen_bitfield_unit.set(18usize, 1u8, {
            let sample_id_all: u64 = unsafe { ::std::mem::transmute(sample_id_all) };
            sample_id_all as u64
        });
        __bindgen_bitfield_unit.set(19usize, 1u8, {
            let exclude_host: u64 = unsafe { ::std::mem::transmute(exclude_host) };
            exclude_host as u64
        });
        __bindgen_bitfield_unit.set(20usize, 1u8, {
            let exclude_guest: u64 = unsafe { ::std::mem::transmute(exclude_guest) };
            exclude_guest as u64
        });
        __bindgen_bitfield_unit.set(21usize, 1u8, {
            let exclude_callchain_kernel: u64 =
                unsafe { ::std::mem::transmute(exclude_callchain_kernel) };
            exclude_callchain_kernel as u64
        });
        __bindgen_bitfield_unit.set(22usize, 1u8, {
            let exclude_callchain_user: u64 =
                unsafe { ::std::mem::transmute(exclude_callchain_user) };
            exclude_callchain_user as u64
        });
        __bindgen_bitfield_unit.set(23usize, 1u8, {
            let mmap2: u64 = unsafe { ::std::mem::transmute(mmap2) };
            mmap2 as u64
        });
        __bindgen_bitfield_unit.set(24usize, 1u8, {
            let comm_exec: u64 = unsafe { ::std::mem::transmute(comm_exec) };
            comm_exec as u64
        });
        __bindgen_bitfield_unit.set(25usize, 1u8, {
            let use_clockid: u64 = unsafe { ::std::mem::transmute(use_clockid) };
            use_clockid as u64
        });
        __bindgen_bitfield_unit.set(26usize, 1u8, {
            let context_switch: u64 = unsafe { ::std::mem::transmute(context_switch) };
            context_switch as u64
        });
        __bindgen_bitfield_unit.set(27usize, 1u8, {
            let write_backward: u64 = unsafe { ::std::mem::transmute(write_backward) };
            write_backward as u64
        });
        __bindgen_bitfield_unit.set(28usize, 1u8, {
            let namespaces: u64 = unsafe { ::std::mem::transmute(namespaces) };
            namespaces as u64
        });
        __bindgen_bitfield_unit.set(29usize, 1u8, {
            let ksymbol: u64 = unsafe { ::std::mem::transmute(ksymbol) };
            ksymbol as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let bpf_event: u64 = unsafe { ::std::mem::transmute(bpf_event) };
            bpf_event as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let aux_output: u64 = unsafe { ::std::mem::transmute(aux_output) };
            aux_output as u64
        });
        __bindgen_bitfield_unit.set(32usize, 1u8, {
            let cgroup: u64 = unsafe { ::std::mem::transmute(cgroup) };
            cgroup as u64
        });
        __bindgen_bitfield_unit.set(33usize, 1u8, {
            let text_poke: u64 = unsafe { ::std::mem::transmute(text_poke) };
            text_poke as u64
        });
        __bindgen_bitfield_unit.set(34usize, 1u8, {
            let build_id: u64 = unsafe { ::std::mem::transmute(build_id) };
            build_id as u64
        });
        __bindgen_bitfield_unit.set(35usize, 1u8, {
            let inherit_thread: u64 = unsafe { ::std::mem::transmute(inherit_thread) };
            inherit_thread as u64
        });
        __bindgen_bitfield_unit.set(36usize, 1u8, {
            let remove_on_exec: u64 = unsafe { ::std::mem::transmute(remove_on_exec) };
            remove_on_exec as u64
        });
        __bindgen_bitfield_unit.set(37usize, 1u8, {
            let sigtrap: u64 = unsafe { ::std::mem::transmute(sigtrap) };
            sigtrap as u64
        });
        __bindgen_bitfield_unit.set(38usize, 26u8, {
            let __reserved_1: u64 = unsafe { ::std::mem::transmute(__reserved_1) };
            __reserved_1 as u64
        });
        __bindgen_bitfield_unit
    }

    pub fn new() -> perf_event_attr
    {
        let bitfield1 = perf_event_attr::new_bitfield_1(
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

        perf_event_attr {
            type_: 0,
            size: 0,
            config: 0,
            sample: perf_event_attr_sample { sample_period: 0 },
            sample_type: 0,
            read_format: 0,
            _bitfield_align_1: [],
            _bitfield_1: bitfield1,
            wakeup: perf_event_attr_wakeup { wakeup_events: 0 },
            bp_type: 0,
            config1: perf_event_attr_config1 { config1: 0 },
            config2: perf_event_attr_config2 { config2: 0 },
            branch_sample_type: 0,
            sample_regs_user: 0,
            sample_stack_user: 0,
            clockid: 0,
            sample_regs_intr: 0,
            aux_watermark: 0,
            sample_max_stack: 0,
            __reserved_2: 0,
            aux_sample_size: 0,
            __reserved_3: 0,
            sig_data: 0,
            config3: 0,
        }
    }
}

pub const __NR_perf_event_open: i64 = 298;  // from asm/unistd_64.h

pub struct PerfEvent
{
    perf_fd: i64,
}

impl PerfEvent
{
    pub fn read(&self, nr: usize) -> Result<Vec<u64>>
    {
        let mut res: Vec<u64> = Vec::with_capacity(nr);
        let res_ptr = res.as_mut_ptr() as *mut libc::c_void;
        let size = nr * mem::size_of::<u64>();

        let ret = unsafe {
            libc::read(self.perf_fd as i32, res_ptr, size) };
        if ret < 0 {
            return Err(io::Error::last_os_error().into());
        }
        unsafe { res.set_len(nr); }

        Ok(res)
    }

    pub fn open(evt_attr: &perf_event_attr,
        pid: i32, cpu: i32, flags: u64) -> Result<PerfEvent>
    {
        let fd = unsafe {
            libc::syscall(__NR_perf_event_open,
                evt_attr, pid, cpu, -1, flags) };
        if fd < 0 {
            return Err(io::Error::last_os_error().into());
        }

        Ok(PerfEvent { perf_fd: fd })
    }
}
