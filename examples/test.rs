use std::time::Instant;
use jobitmap::batchbitmap::BatchBitmap;
use jobitmap::{Bitmap, BitmapI, ByteBufferResource};
use jobitmap::mmap::{SegmentArray, SegmentContainer};

fn main() {
    let br = ByteBufferResource::new("./index".to_string(), false);
    let segment_array = SegmentArray::new(br);

    let br = ByteBufferResource::new("./data".to_string(), false);
    let data_container = SegmentContainer::new(br, 256*64);

    let batch_bitmap = BatchBitmap::new(segment_array, data_container);
    let mut bitmap = Bitmap::new(batch_bitmap);
    let start_time = Instant::now();

    for i in 0..1000000000 {
        bitmap.set(i, true);
        assert!(bitmap.get(i));
    }

    for i in 0..1000000000 {
        bitmap.set(i, false);
        assert!(bitmap.get(i));
    }

    let elapsed_time = start_time.elapsed();

    // 打印执行时间（以毫秒为单位）
    println!("Elapsed time: {} ms", elapsed_time.as_millis());
}