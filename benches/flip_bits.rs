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

fn flip_small(c: &mut Criterion) {
        let orig = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 0, 
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let rle = RLE::from(&orig);
        c.bench_function("flip_small", move |b| {
            b.iter(|| rle.flip_bits())
        });
}

fn flip_small_iter(c: &mut Criterion) {
        let orig = Image::new(6, 6, vec![
            0, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 1, 0, 
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0,
        ]);
        let rle = RLE::from(&orig);
        c.bench_function("flip_small_iter", move |b| {
            b.iter(|| rle.flip_bits_iter().collect::<Vec<_>>())
        });
}


fn flip_4k(c: &mut Criterion) {
        let orig = clone_to_image(&load_image("benches/slice000.png"));
        let erode = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        c.bench_function("flip_4k", move |b| {
            b.iter(|| rle.flip_bits())
        });
}

fn flip_4k_iter(c: &mut Criterion) {
        let orig = clone_to_image(&load_image("benches/slice000.png"));
        let erode = Image::new(3, 3, vec![
            0, 1, 0,
            1, 1, 1,
            0, 1, 0
        ]);
        let rle = RLE::from(&orig);
        c.bench_function("flip_4k_iter", move |b| {
            b.iter(|| rle.flip_bits_iter().collect::<Vec<_>>())
        });
}

criterion_group! {
    flip, flip_4k, flip_small, flip_small_iter, flip_4k_iter
}
criterion_main!(flip);
