use rle_morph::{Image, Run, RLE};
use std::path::Path;
use imageproc::distance_transform::Norm;
use imageproc::morphology::{dilate, dilate_mut};
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

fn dilate_rle(img: Image) -> Image {
    let mut rle = RLE::from(&img);
    let dilated = rle.dilate(&RLE::l1_structuring(1)).to_image();
    let (w, h) = (dilated.w(), dilated.h());
    let mut img_data = dilated.into_raw();
    // binary image is 0s and 1s, convert all 1s to white
    for val in &mut img_data {
        *val = if *val > 0 {
            255
        } else {
            0
        };
    }
    Image::new(w, h, img_data)
}

fn dilate_imageproc(mut img: GrayImage) -> GrayImage {
    dilate_mut(&mut img, Norm::L1, 1);
    img
}

fn main() {
    let gray_image = load_image(std::env::args().skip(1).next().expect("provide path to png image"));

    // dilate using rle
    let img: Image = clone_to_image(&gray_image);
    let start = std::time::Instant::now();
    let img = dilate_rle(img);
    println!("Time took to dilate with rle: {} ms", start.elapsed().as_millis());
    clone_to_gray_image(&img).save("rle_dilate.png").unwrap();

    // dilate using imageproc
    let start = std::time::Instant::now();
    let gray = dilate_imageproc(gray_image);
    println!("Time took to dilate with imageproc: {} ms", start.elapsed().as_millis());
    gray.save("imageproc_dilate.png").unwrap();
}
