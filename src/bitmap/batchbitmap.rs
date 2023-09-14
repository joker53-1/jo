use crate::{mmap::{SegmentArray, SegmentContainer, SegmentEntry}, ByteOperat};

use super::BatchBitmapI;

#[derive(Clone)]
struct BitmapSegment {
    segment_entry: SegmentEntry,
}

impl Default for BitmapSegment {
    fn default() -> Self {
        Self { segment_entry: Default::default() }
    }
}

pub struct BatchBitmap{
    segment_array: SegmentArray,
    data_container: SegmentContainer,
    // offset: usize,
    default: bool,
    last_get_segment_entry_index: usize,
    last_get_segment: BitmapSegment
}

impl BatchBitmap {
    pub fn new(segment_array: SegmentArray, data_container: SegmentContainer) -> Self {
        Self { segment_array: segment_array, data_container: data_container, default: false, last_get_segment_entry_index: usize::MAX, last_get_segment: BitmapSegment::default() }
    }

    fn get_segment(&mut self, segment_idx: usize)-> BitmapSegment {
        if self.last_get_segment_entry_index!= segment_idx {
            let seg_entry = self.segment_array.get(segment_idx);
            if seg_entry.offset == 0xFFFF_FFFF_FFFF {
                BitmapSegment::default()
            }else {
                BitmapSegment{ segment_entry: seg_entry}
            }
        }else {
            self.last_get_segment.clone()
        }
    }

    fn create_and_fill_segment(&mut self, seg_idx: usize) -> BitmapSegment {
        let offset = self.data_container.alloc();
        let mut binding = self.data_container.get_segment();
        let buffer = binding.get_slice(offset);
        let seg_entry = SegmentEntry{ offset: offset.try_into().unwrap(), default_value_count: 0x100 -1 };
        self.segment_array.set(seg_idx, seg_entry.clone());
        let seg = BitmapSegment{segment_entry:seg_entry};
        if !self.default {
            buffer.fill(0);
        }else {
            buffer.fill(0xFF);
        }
        self.last_get_segment_entry_index = seg_idx;
        self.last_get_segment = seg.clone();
        return seg;
    }

}

impl BatchBitmapI for BatchBitmap {
    fn set(&mut self, idx: usize, value: u64) {
        let seg_idx = idx >> 8;
        let mut seg = self.get_segment(seg_idx);
        let inner_seg_idx = idx & 0xFF;
        if seg.segment_entry.offset == 0xFFFF_FFFF_FFFF {
            if value != 0 {
                seg = self.create_and_fill_segment(seg_idx);
                let mut binding = self.data_container.get_segment();
                let mut buffer = binding.get_slice(seg.segment_entry.offset.try_into().unwrap());
                buffer.put_u64(inner_seg_idx << 3, value);
                return;
            }
        }else {
            let mut binding = self.data_container.get_segment();
            let mut buffer = binding.get_slice(seg.segment_entry.offset.try_into().unwrap());
            let cur_u64 = buffer.get_u64(inner_seg_idx << 3);
            if value != cur_u64 {
                buffer.put_u64(inner_seg_idx << 3, value);
                let old_count = seg.segment_entry.default_value_count;
                let mut new_count = old_count;
                if cur_u64 == 0 {
                    new_count = old_count -1;
                }else if value == 0 {
                    new_count = old_count +1;
                }

                if old_count != new_count {
                    if new_count == 256 {
                        self.last_get_segment = BitmapSegment::default();
                        self.segment_array.set(seg_idx, SegmentEntry { offset: 0xFFFF_FFFF_FFFF, default_value_count: 255 })
                        //todo release
                    }else {
                        let tmp =SegmentEntry { offset: seg.segment_entry.offset, default_value_count: new_count };
                        self.segment_array.set(seg_idx, tmp.clone());
                        self.last_get_segment.segment_entry = tmp;
                    }
                }
            }
        }
    }

    fn get(&mut self, idx: usize) -> u64 {
        let seg = self.get_segment(idx/256 );
        if seg.segment_entry.offset == 0xFFFF_FFFF_FFFF {
            return 0;
        }else {
            let mut binding = self.data_container.get_segment();
            let buffer = binding.get_slice(seg.segment_entry.offset.try_into().unwrap());
            return buffer.get_u64((idx & 0xFF)<<3);
        }
    }

}


#[cfg(test)]
mod test{
    use crate::{ByteBufferResource, mmap::{SegmentArray, SegmentContainer}, bitmap::BatchBitmapI};

    use super::BatchBitmap;

    #[test]
    fn test(){
        let br = ByteBufferResource::new("./index".to_string(), false);
        let segment_array = SegmentArray::new(br);

        let br = ByteBufferResource::new("./data".to_string(), false);
        let data_container = SegmentContainer::new(br, 256*64);

        let mut batch_bitmap = BatchBitmap::new(segment_array, data_container);
        batch_bitmap.set(0, u64::MAX - 1);
        batch_bitmap.set(18, 9007199254740991);
        batch_bitmap.set(19, 900719925474099);

        assert_eq!(u64::MAX-1, batch_bitmap.get(0));
        assert_eq!(9007199254740991, batch_bitmap.get(18));
        assert_eq!(900719925474099, batch_bitmap.get(19));

        assert_eq!(0, batch_bitmap.get(1));
        assert_eq!(0, batch_bitmap.get(256));
        batch_bitmap.set(256, 18);
        assert_eq!(18, batch_bitmap.get(256));

        assert_eq!(253, batch_bitmap.segment_array.get(0).default_value_count);
    }

}