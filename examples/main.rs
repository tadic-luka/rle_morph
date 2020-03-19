use rle_morph::{Image, Run, RLE};
use std::path::Path;

fn run_dilation() {
    let mut a = RLE::new(7, 7);
    a.add_run(Run {
        x_start: 3,
        x_end: 4,
        y: 0,
    });
    a.add_run(Run {
        x_start: 3,
        x_end: 4,
        y: 3,
    });
    println!("Before dilation:\n{}", a.to_image());
    let a = a.dilate(&RLE::l1_structuring(1));
    println!("After dilation:\n{}", a.to_image());

}
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
fn main() {
    let img = load_image(std::env::args().skip(1).next().expect("provide path to png image"));
    let start = std::time::Instant::now();
    println!("Image memory: {} B", img.data().len());
    let mut a = RLE::from(img);
    println!("Time took to load img: {} ms", start.elapsed().as_millis());
    println!("Total runs: {}", a.runs().len());
    println!("RLE memory: {} B", a.runs().len() * std::mem::size_of::<Run>());
}
