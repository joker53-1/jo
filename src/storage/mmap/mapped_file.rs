use std::fs::{File, OpenOptions};

use memmap2::{Mmap, MmapOptions, MmapMut};

use super::{MappedResource, bytebuffer::ByteBuffer};


pub struct MappedFile {
    filepath: String,
    file: File
}

pub enum MmapT {
    Mmap(Mmap),
    MmapMut(MmapMut)
}

impl MappedFile {
    pub(crate) fn new(filepath: String) -> Self {
        let file = OpenOptions::new().read(true).write(true).create(true).open(&filepath).unwrap();
        Self { filepath, file}
    }

    pub(crate) fn force(&self) {
        self.file.sync_all().unwrap();
    }
}

impl MappedResource for MappedFile {
    type BufferType = ByteBuffer;

    fn map(&self, position: u64, size: usize, is_readonly: bool) -> Result<Self::BufferType, ()> {
        let mmap_file = if is_readonly {
            let mmap = unsafe {
                MmapOptions::new().offset(position).len(size).map(&self.file).unwrap()
            }; 
            MmapT::Mmap(mmap)
        }else {
            let _ = self.file.set_len(size.try_into().unwrap());
            let mmap = unsafe { MmapOptions::new().offset(position).len(size).map_mut(&self.file).unwrap() };
            MmapT::MmapMut(mmap)
        };
        Ok(ByteBuffer::new(mmap_file))
    }

}
