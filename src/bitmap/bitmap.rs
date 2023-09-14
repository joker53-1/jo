use super::{batchbitmap::BatchBitmap, BatchBitmapI, BitmapI, Flushable};


pub struct Bitmap{
    batch_bitmap: BatchBitmap,
    wait_update_idx: usize,
    wait_update_u64: u64
}

impl Bitmap {
    pub fn new(batch_bitmap: BatchBitmap) -> Self{
        Bitmap { batch_bitmap, wait_update_idx: usize::MAX, wait_update_u64: 0}
    }

    fn get_u64(&mut self, idx: usize) -> u64{
        if idx == self.wait_update_idx {
            self.wait_update_u64
        } else {
            self.batch_bitmap.get(idx)
        }
    }

    fn set_u64(&mut self, idx: usize, update_u64: u64) {
        if idx != self.wait_update_idx {
            self.flush_wait_u64();
            self.wait_update_idx = idx;
        }
        self.wait_update_u64 = update_u64;
    }

    fn flush_wait_u64(&mut self){
        if self.wait_update_idx != usize::MAX {
            self.batch_bitmap.set(self.wait_update_idx, self.wait_update_u64);
            self.wait_update_idx = usize::MAX;
            self.wait_update_u64 = 0;
        }
    }

    fn set_true(&mut self, idx: usize){
        let u64_idx = idx >> 6;
        let cur_u64 = self.get_u64(u64_idx);
        let bit_off = idx & 0x3F;
        if cur_u64 == u64::MAX || cur_u64.get_bit(bit_off) {
            return;
        }
        let modified_u64 = cur_u64.set_true(bit_off);
        self.set_u64(u64_idx, modified_u64);
    }

    fn set_false(&mut self, idx: usize){
        let u64_idx = idx >> 6;
        let cur_u64 = self.get_u64(u64_idx);
        let bit_off = idx & 0x3F;
        if cur_u64== 0 || !cur_u64.get_bit(bit_off) {
            return;
        }
        let modified_u64 = cur_u64.set_false(bit_off);
        self.set_u64(u64_idx, modified_u64);
    }

}

trait BitOper {
    fn get_bit(&self, bit_off: usize)-> bool;
    fn set_true(&self, bit_off: usize) -> Self;
    fn set_false(&self, bit_off: usize) -> Self;
}

impl BitOper for u64 {
    fn get_bit(&self, bit_off: usize)-> bool {
        (self & (1<< bit_off)) !=0
    }

    fn set_true(&self, bit_off: usize) -> Self {
        self | (1 << bit_off)
    }

    fn set_false(&self, bit_off: usize) -> Self {
        self & !(1<< bit_off)
    }
    
}

impl BitmapI for Bitmap {
    fn set(&mut self, idx: usize, value: bool) {
        if value {
            self.set_true(idx)
        }else {
            self.set_false(idx)
        }
    }

    fn get(&mut self, idx: usize) -> bool {
        let u64_idx = idx >> 6;
        let cur_u64 = self.get_u64(u64_idx);
        let bit_off = idx & 0x3F;
        return cur_u64.get_bit(bit_off);
    }

    
}

impl Flushable for Bitmap {
    fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.flush_wait_u64();
        return Ok(());
    }
}

mod test{
    use crate::{ByteBufferResource, mmap::{SegmentArray, SegmentContainer}, bitmap::BitmapI};

    use super::{BatchBitmap, Bitmap};

    #[test]
    fn test(){
        let br = ByteBufferResource::new("./index".to_string(), false);
        let segment_array = SegmentArray::new(br);

        let br = ByteBufferResource::new("./data".to_string(), false);
        let data_container = SegmentContainer::new(br, 256*64);

        let batch_bitmap = BatchBitmap::new(segment_array, data_container);
        
        let mut bimap = Bitmap::new(batch_bitmap);
        for i in 0..1000 {
            bimap.set(i, true);
            assert!(bimap.get(i));
        }
        for i in 0..1000 {
            bimap.set(i, false);
            assert!(!bimap.get(i));
        }
        for i in 0..100 {
            bimap.set(1000+16384*i, true);
            assert!(bimap.get(1000+16384*i));
        }
        for i in 0..100 {
            bimap.set(1000+16384*i, false);
            assert!(!bimap.get(1000+16384*i));
        }
    }

}