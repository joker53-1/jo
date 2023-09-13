use crate::{BaseResource, ByteBufferResource, ByteOperat};

pub struct SegmentArray {
    buffer: ByteBufferResource,
}

impl SegmentArray {
    fn new(buffer: ByteBufferResource) -> Self {
        return Self{ buffer };
    }

    fn get(&mut self, idx: usize) -> SegmentEntry {
        let pos = idx * 8;
        if &self.buffer.len - pos < 8 {
            return SegmentEntry::default();
        }
        let _ = &self.buffer.map_content();
        let seg = self.buffer.content_buffer.get_u64(pos);
        return SegmentEntry {
            offset: seg & 0xFFFF_FFFF_FFFF,
            default_value_count: (seg as usize>> 48) & 0xFF,
        };
    }

    fn set(&mut self, idx: usize, value: SegmentEntry) {
        let min_capacity = (idx + 1) * 8;
        if self.buffer.len < min_capacity {
            self.buffer.resize(get_next_capacity(min_capacity));
        }
        let pos = idx * 8;
        let seg = value.offset | (value.default_value_count as u64) << 48;
        let _ = &self.buffer.map_content();
        self.buffer.content_buffer.put_u64(pos, seg)
    }
}


fn get_next_capacity(min_capacity: usize) -> usize {
    if min_capacity < 1024 * 16 {
        min_capacity
    } else {
        min_capacity + (1024 * 16)
    }
}

struct SegmentEntry {
    offset: u64,
    default_value_count: usize,
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

        assert_eq!(0, segment_entry_array.get(10).offset);
        assert_eq!(0, segment_entry_array.get(101).offset);
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
