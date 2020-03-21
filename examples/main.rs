use rle_morph::{Image, Run, RLE};
use std::path::Path;

fn load_image<P: AsRef<Path>>(path: P) -> Image {
    let img = image::open(path).unwrap();
    let img = img.into_luma();
    let (width, height) = (img.width(), img.height());

    println!("dimensions: {}x{}", width, height);
    Image::new(width as _, height as _, img.into_raw())
}
fn main() {
    let img = load_image(std::env::args().skip(1).next().expect("provide path to png image"));
    let start = std::time::Instant::now();
    let mut rle = RLE::from(&img);
    let start = std::time::Instant::now();
    let dilated = rle.dilate(&RLE::l1_structuring(1)).to_image();
    println!("Time took to dilate: {} ms", start.elapsed().as_millis());
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
    let gray_image = image::GrayImage::from_raw(w as _, h as _, img_data).unwrap();
    gray_image.save("output.png").unwrap();
}
