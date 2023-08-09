

use super::{mapped_file::MmapT, ByteOperat};


pub struct ByteBuffer {
    pub(crate) mmap_file: MmapT
}

impl ByteBuffer {
    pub fn new(mmap_file: MmapT) ->Self {
        Self { mmap_file }
    }
}

impl ByteOperat for ByteBuffer {
    unsafe fn get_byte(&self, position: usize) -> u8 {
        match &self.mmap_file {
            MmapT::Mmap(mm) => {
                mm[position]
            },
            MmapT::MmapMut(mmt) => {
                mmt[position]
            }
        }
    }

    unsafe fn put_byte(self, position: usize, value: u8) {
        match self.mmap_file {
            MmapT::Mmap(_) => {
                panic!("Unsupport!");
            },
            MmapT::MmapMut(mut mmt) => {
                mmt[position] = value
            }
        }
    }

    unsafe fn get_i64(&self, position: usize) -> i64 {
        match &self.mmap_file {
            MmapT::Mmap(mm) => {
                let ptr = mm.as_ptr().add(position) as *const i64;
                *ptr
            },
            MmapT::MmapMut(mmt) => {
                let ptr = mmt.as_ptr().add(position) as *const i64;
                *ptr
            }
        }
    }

    unsafe fn put_i64(self, position: usize, value: i64) {
        match self.mmap_file {
            MmapT::Mmap(_) => {
                panic!("Unsupport!");
            },
            MmapT::MmapMut(mut mmt) => {
                let ptr = mmt.as_mut_ptr().add(position) as *mut i64;
                *ptr = value;
            }
        }
    }

    
}