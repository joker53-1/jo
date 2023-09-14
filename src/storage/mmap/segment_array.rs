use crate::{BaseResource, ByteBufferResource, ByteOperat};

pub struct SegmentArray {
    buffer: ByteBufferResource,
}

impl SegmentArray {
    pub fn new(buffer: ByteBufferResource) -> Self {
        return Self{ buffer };
    }

    pub fn get(&mut self, idx: usize) -> SegmentEntry {
        let pos:i64 = (idx * 8) as i64;
        if &self.buffer.len.try_into().unwrap() - pos < 8 {
            return SegmentEntry::default();
        }
        let _ = &self.buffer.map_content();
        let bf = self.buffer.content_buffer.as_ref().unwrap();
        let seg = bf.get_u64(pos as usize);
        return SegmentEntry {
            offset: seg & 0xFFFF_FFFF_FFFF,
            default_value_count: (seg as usize>> 48) & 0xFF,
        };
    }

    pub fn set(&mut self, idx: usize, value: SegmentEntry) {
        let min_capacity = (idx + 1) * 8;
        if self.buffer.len < min_capacity {
            let old_capacity = self.buffer.len;
            self.buffer.resize(get_next_capacity(min_capacity));
            self.buffer.fill(old_capacity, min_capacity-old_capacity, u8::MAX);
        }
        let pos = idx * 8;
        let seg = value.offset | (value.default_value_count as u64) << 48;
        let _ = &self.buffer.map_content();
        self.buffer.content_buffer.as_mut().unwrap().put_u64(pos, seg)
    }
}


fn get_next_capacity(min_capacity: usize) -> usize {
    if min_capacity < 1024 * 16 {
        min_capacity
    } else {
        min_capacity + (1024 * 16)
    }
}

#[derive(Clone)]
pub struct SegmentEntry {
    pub offset: u64,
    pub default_value_count: usize,
}

impl Default for SegmentEntry {
    fn default() -> Self {
        Self {
            offset: 0xFFFF_FFFF_FFFF,
            default_value_count: 0x100,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ByteBufferResource, storage::mmap::segment_array::SegmentEntry};

    use super::SegmentArray;

    #[test]
    fn test() {
        let br = ByteBufferResource::new("/tmp/test".to_string(), false);
        let mut segment_entry_array = SegmentArray::new(br);
        segment_entry_array.set(100,  SegmentEntry { offset: 12, default_value_count: 12 });
    
        assert_eq!(12, segment_entry_array.get(100).offset);
        assert_eq!(12, segment_entry_array.get(100).default_value_count);

        assert_eq!(0xFFFF_FFFF_FFFF, segment_entry_array.get(10).offset);
        assert_eq!(0xFFFF_FFFF_FFFF, segment_entry_array.get(101).offset);
    }

    #[test]
    fn test1(){
        let br = ByteBufferResource::new("/tmp/test".to_string(), false);
        let mut segment_entry_array = SegmentArray::new(br);
    
        for i in 0..=100 {
            segment_entry_array.set(i, SegmentEntry { offset: 12, default_value_count: 12 });
        }
        
        for i in 0..=100 {
            assert_eq!(12, segment_entry_array.get(i).offset);
            assert_eq!(12, segment_entry_array.get(i).default_value_count);
        }
    }
}
