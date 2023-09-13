
use super::{mapped_file::MmapT, ByteOperat};

pub struct ByteBuffer {
    pub(crate) mmap_file: MmapT,
}

impl ByteBuffer {
    pub fn new(mmap_file: MmapT) -> Self {
        Self { mmap_file }
    }

}

#[inline]
fn handle_read<T, F>(mmap_file: &MmapT, position: usize, immutable_read: F) -> T
where
    F: Fn(&[u8], usize) -> T,
{
    match mmap_file {
        MmapT::Mmap(mm) => immutable_read(mm, position),
        MmapT::MmapMut(mmt) => immutable_read(&mmt, position),
    }
}

#[inline]
fn handle_write<T, F>(mmap_file: &mut MmapT, position: usize, value: T, mutable_write: F)
where
    F: Fn(&mut [u8], usize, T),
{
    match mmap_file {
        MmapT::Mmap(_) => panic!("Unsupport!"),
        MmapT::MmapMut(mmt) => mutable_write(mmt.as_mut(), position, value),
    }
}

impl ByteOperat for ByteBuffer {
    fn get_byte(&self, position: usize) -> u8 {
        handle_read(&self.mmap_file, position, |mm, position| mm[position])
    }

    fn put_byte(&mut self, position: usize, value: u8) {
        handle_write(&mut self.mmap_file, position, value, |mmt, position, value| {
            mmt[position] = value
        });
    }

    fn get_u64(&self, position: usize) -> u64 {
        let get_fn = |mm: &[u8], position| unsafe {
            let ptr = mm.as_ptr().add(position) as *const u64;
            *ptr
        };
        handle_read(&self.mmap_file, position, get_fn)
    }

    fn put_u64(&mut self, position: usize, value: u64) {
        let put_fn = |mmt: &mut [u8], position: usize, value: u64| unsafe {
            let ptr = mmt.as_mut_ptr().add(position) as *mut u64;
            *ptr = value;
        };
        handle_write(&mut self.mmap_file, position, value, put_fn);
    }
}
