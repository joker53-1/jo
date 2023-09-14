use crate::{ByteBufferResource, BaseResource, MappedResource, ByteOperat};

use super::{MmapT, ByteBuffer};

struct SegmentMeta(usize);

pub struct SegmentContainer {
    buffer: ByteBufferResource,
    metadata: SegmentMeta,
}


impl SegmentContainer {
    pub fn new(buffer: ByteBufferResource, page_size: usize) -> Self {
        return Self{ buffer, metadata: SegmentMeta(page_size) };
    }

    pub fn alloc(&mut self) -> usize {
        let old_len = self.buffer.len;
        let buffer_size = self.buffer.len + self.metadata.0;
        self.buffer.resize(buffer_size);
        return old_len;
    }

    pub fn get_segment(&mut self) -> SnapshotSegmentContainer{
        match &mut self.buffer.content_buffer.as_mut().unwrap().mmap_file {
            MmapT::Mmap(_) => unimplemented!(),
            MmapT::MmapMut(mmt) => SnapshotSegmentContainer { slice: mmt, page_size: self.metadata.0 },
        }
    }

    pub fn get_mut_buffer(&self, pos: usize) -> ByteBuffer {
        self.buffer.mapped_file.map(pos.try_into().unwrap(), self.metadata.0, false).unwrap() 
    }
}

pub struct SnapshotSegmentContainer<'a>{
    slice:  &'a mut [u8],
    page_size: usize
}

impl SnapshotSegmentContainer<'_> {
    pub fn get_slice(&mut self, pos: usize) -> &mut [u8] {
        &mut self.slice[pos..pos+self.page_size]
    }
}

impl ByteOperat for &mut [u8] {
    fn get_byte(&self, position: usize) -> u8 {
       self[position]
    }

    fn put_byte(&mut self, position: usize, value: u8) {
        self[position] = value;
    }

    fn get_u64(&self, position: usize) -> u64 {
        unsafe {
            let ptr = self.as_ptr().add(position) as *const u64;
            *ptr
        }
    }

    fn put_u64(&mut self, position: usize, value: u64) {
        unsafe {
        let ptr = self.as_mut_ptr().add(position) as *mut u64;
            *ptr = value;
        };
    }
}



#[cfg(test)]
mod test {
    use crate::{ByteBufferResource, ByteOperat};

    use super::SegmentContainer;

   
    #[test]
    fn test() {
        let br = ByteBufferResource::new("./test".to_string(), false);
        let mut seg_container = SegmentContainer::new(br, 256*64);
        for _ in 1..10 {
            seg_container.alloc();
        }
        let mut snap = seg_container.get_segment();

        let mut s = snap.get_slice(0);
        s[0] = 11;
        s.put_u64(8, 67777);
        assert_eq!(s[0], 11);
        assert_eq!(unsafe {
            let ptr = s.as_ptr().add(8) as *const u64;
            *ptr
        }, 67777);
        let mut s = snap.get_slice(5*16384);
        s[1] = 100;
        s.put_u64(8, 67777);
        assert_eq!(s[1], 100);
        assert_eq!(unsafe {
            let ptr = s.as_ptr().add(8) as *const u64;
            *ptr
        }, 67777);

    }

}
