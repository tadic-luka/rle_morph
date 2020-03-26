use criterion::{black_box, criterion_group, criterion_main, Criterion, BatchSize};
use std::path::Path;
use rle_morph::{Run, RLE, Image};
use image::GrayImage;

fn load_image<P: AsRef<Path>>(path: P) -> GrayImage {
    image::open(path).unwrap().into_luma()
}

fn clone_to_image(img: &GrayImage) -> Image {
    let clone = img.clone();
    let (width, height) = img.dimensions();
    Image::new(width as _, height as _, clone.into_raw())
}

fn clone_to_gray_image(img: &Image) -> GrayImage {
    let clone = img.clone();
    let (width, height) = (img.w(), img.h());
    GrayImage::from_raw(width as _, height as _, clone.into_raw()).unwrap()
}

fn erode_small_rle(c: &mut Criterion) {
        let orig = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 0, 
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let erode = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let erode = RLE::from(&erode);
        c.bench_function("erode_small_rle", move |b| {
            b.iter(|| rle.erode(&erode))
        });
}

fn erode_small_image_crate(c: &mut Criterion) {
    use imageproc::distance_transform::Norm;
    use imageproc::morphology::erode_mut;
    let orig = image::GrayImage::from_raw(6, 6, vec![
        0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 1, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0
    ]).unwrap();
    c.bench_function("erode_small_image_crate", move |b| {
            b.iter_batched(|| orig.clone(),
                |mut img| erode_mut(&mut img, Norm::L1, 1),  BatchSize::SmallInput
            )
    });
}

fn erode_4k_rle(c: &mut Criterion) {
        let orig = clone_to_image(&load_image("benches/slice000.png"));
        let erode = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        let erode = RLE::from(&erode);
        c.bench_function("erode_4k_rle", move |b| {
            b.iter(|| rle.erode(&erode))
        });
}

fn erode_4k_image_crate(c: &mut Criterion) {
        use imageproc::distance_transform::Norm;
        use imageproc::morphology::erode_mut;
        let orig = load_image("benches/slice000.png");
        c.bench_function("erode_4k_image_crate", move |b| {
            b.iter_batched(|| orig.clone(),
                |mut img| erode_mut(&mut img, Norm::L1, 1),  BatchSize::LargeInput
            )
        });
}

criterion_group! {
    erode, erode_small_rle,erode_4k_rle, erode_small_image_crate,
    erode_4k_image_crate
}

criterion_main!(erode);
