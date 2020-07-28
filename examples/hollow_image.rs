use rle_morph::{Image, Run, RLE};
use std::path::Path;
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate_mut, erode_mut};
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
    let img_data = clone.into_raw();
    GrayImage::from_raw(width as _, height as _, img_data).unwrap()
}

fn holow_rle(img: RLE) -> RLE {
    let mut eroded = img.erode(&RLE::linf_structuring(5));
    &img - &eroded
}

fn hollow_imageproc(img: &mut GrayImage) {
    let mut clone = img.clone();
    erode_mut(&mut clone, Norm::LInf, 5);
    for (pix_mut, pix) in img.pixels_mut().zip(clone.pixels()) {
        pix_mut[0] = pix_mut[0] ^ pix[0];
    }
}


fn print_usage_and_exit() {
    eprintln!("Usage: prog /path/to/image");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args().skip(1);
    let file = match args.next() {
        Some(file) => file,
        None => {
            print_usage_and_exit();
            unreachable!();
        },
    };
    let operation = args.next().unwrap_or("d".to_string());
    let mut gray_image = load_image(file);

    let mut img = clone_to_image(&gray_image);
    let rle = RLE::from(&img);

    let start = std::time::Instant::now();
    let tmp = holow_rle(rle);
    println!("Time took to hollow with rle: {} us", start.elapsed().as_micros());
    img = tmp.to_image(255);

    let start = std::time::Instant::now();
    hollow_imageproc(&mut gray_image);
    println!("Time took to hollow with imageproc: {} us", start.elapsed().as_micros());
    clone_to_gray_image(&img).save("rle_hollow.png").unwrap();
    gray_image.save("imageproc_hollow.png").unwrap();
}
