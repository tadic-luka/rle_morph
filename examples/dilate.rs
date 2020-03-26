use rle_morph::{Image, Run, RLE};
use std::path::Path;
use imageproc::distance_transform::Norm;
use imageproc::morphology::dilate_mut;
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
    let mut img_data = clone.into_raw();
    // binary image is 0s and 1s, convert all 1s to white
    for val in &mut img_data {
        *val = if *val > 0 {
            255
        } else {
            0
        };
    }
    GrayImage::from_raw(width as _, height as _, img_data).unwrap()
}

fn main() {
    let mut gray_image = load_image(std::env::args().skip(1).next().expect("provide path to png image"));

    // dilate using rle
    let img: Image = clone_to_image(&gray_image);
    let rle = RLE::from(&img);
    let start = std::time::Instant::now();
    let img = rle.dilate(&RLE::linf_structuring(5)).to_image();
    println!("Time took to dilate with rle and decode: {} us", start.elapsed().as_micros());
    clone_to_gray_image(&img).save("rle_dilate.png").unwrap();

    // dilate using imageproc
    let start = std::time::Instant::now();
    dilate_mut(&mut gray_image, Norm::LInf, 5);
    println!("Time took to dilate with imageproc: {} us", start.elapsed().as_micros());
    gray_image.save("imageproc_dilate.png").unwrap();
}
