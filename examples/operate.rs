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


fn print_usage_and_exit() {
    eprintln!(r#"Usage: prog /path/to/image [eE|dD]

        [eE|dD] - erode (e) or dilate (d) image, default d (dilate)"#);
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

    match operation.as_ref() {
        "d" | "D" => {
            let start = std::time::Instant::now();
            img = rle.dilate(&RLE::linf_structuring(5)).to_image(255);
            println!("Time took to dilate with rle and decode: {} us", start.elapsed().as_micros());

            let start = std::time::Instant::now();
            dilate_mut(&mut gray_image, Norm::LInf, 5);
            println!("Time took to dilate with imageproc: {} us", start.elapsed().as_micros());
        },
        "e" | "E" => {
            let start = std::time::Instant::now();
            img = rle.erode(&RLE::linf_structuring(5)).to_image(255);
            println!("Time took to erode with rle and decode: {} us", start.elapsed().as_micros());

            let start = std::time::Instant::now();
            erode_mut(&mut gray_image, Norm::LInf, 5);
            println!("Time took to erode with imageproc: {} us", start.elapsed().as_micros());
        },
        _ => {
            print_usage_and_exit();
        }
    }
    clone_to_gray_image(&img).save("rle_operation.png").unwrap();
    gray_image.save("imageproc_operation.png").unwrap();
}
