use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use std::path::Path;
use rle_morph::{Run, RLE, Image};

fn load_image<P: AsRef<Path>>(path: P) -> Image {
    use png::ColorType::*;
    use std::fs::File;
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data).unwrap();

    let data = match info.color_type {
        Grayscale => {
            img_data
        }
        _ => panic!("Invalid png type"),
    };
    println!("dimensions: {}x{}", info.width, info.height);
    Image::new(info.width as _, info.height as _, data)
}


fn dilate_small_rle(c: &mut Criterion) {
        let orig = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 0, 
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let dilate = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let dilate = RLE::from(&dilate);
        c.bench_function("small_rle", move |b| {
            b.iter(|| rle.dilate(&dilate))
        });
}

fn dilate_small_image_crate(c: &mut Criterion) {
    use imageproc::distance_transform::Norm;
    use imageproc::morphology::dilate_mut;
    let orig = image::GrayImage::from_raw(6, 6, vec![
        0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0
    ]).unwrap();
    c.bench_function("small_image_crate", move |b| {
            b.iter_batched(|| orig.clone(),
                |mut img| dilate_mut(&mut img, Norm::L1, 1),  BatchSize::SmallInput
            )
    });
}

fn dilate_4k_rle(c: &mut Criterion) {
        let orig = load_image("benches/slice000.png");
        let dilate = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let dilate = RLE::from(&dilate);
        c.bench_function("4k_rle", move |b| {
            b.iter(|| rle.dilate(&dilate))
        });
}

fn dilate_4k_image_crate(c: &mut Criterion) {
        use imageproc::distance_transform::Norm;
        use imageproc::morphology::dilate_mut;
        let orig = image::open("benches/slice000.png").unwrap();
        let orig = orig.into_luma();
        c.bench_function("4k_image_crate", move |b| {
            b.iter_batched(|| orig.clone(),
                |mut img| dilate_mut(&mut img, Norm::L1, 1),  BatchSize::LargeInput
            )
        });
}

criterion_group! {
    dilate, dilate_small_rle,dilate_4k_rle, dilate_small_image_crate, dilate_4k_image_crate
}

criterion_main!(dilate);
