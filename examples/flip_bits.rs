use rle_morph::{Image, Run, RLE};
use std::path::Path;
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate, dilate_mut, erode_mut};
use image::GrayImage;

fn load_image<P: AsRef<Path>>(path: P) -> Image {
    let gray = image::open(path).unwrap().into_luma();
    let (w, h) = gray.dimensions();
    Image::new(w as _, h as _, gray.into_raw())
}

fn to_gray_image(img: Image) -> GrayImage {
    let (width, height) = (img.w(), img.h());
    let mut raw = img.into_raw();
    for val in &mut raw {
        *val = if *val > 0 {
            255
        } else {
            0
        };
    }
    GrayImage::from_raw(width as _, height as _, raw).unwrap()
}


fn main() {
    let file = std::env::args().skip(1).next().expect("provide path to png image");
    let img = load_image(file);
    let mut rle = RLE::from(&img);

    // flip bits
    let start = std::time::Instant::now();
    let img = rle.flip_bits().to_image(255);
    println!("Time took to flip bits: {} us", start.elapsed().as_micros());
    to_gray_image(img).save("flipped.png").unwrap();
}
