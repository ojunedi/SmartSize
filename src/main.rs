pub mod matrix;
pub mod image;
pub mod processing;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::env;
use std::io;
use std::fs::File;
use std::io::Write;

pub struct Config<'a> {
    input_file: &'a str,
    output_file: &'a str,
    desired_width: i32,
    desired_height: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_file = &args[2];
    let desired_width = args[3].parse::<i32>().unwrap();
    let desired_height = args[4].parse::<i32>().unwrap();
    let image_config = Config {
        input_file,
        output_file,
        desired_width,
        desired_height
    };
    let mut img: &mut Box<image::Image> = &mut Box::new(image::Image {
        width: 0,
        height: 0,
        red_channel:   Box::new(matrix::Matrix::default()),
        blue_channel:  Box::new(matrix::Matrix::default()),
        green_channel: Box::new(matrix::Matrix::default())
    });

    let mut energy: Box<matrix::Matrix> = Box::new(matrix::Matrix::default());
    let mut   cost: Box<matrix::Matrix> = Box::new(matrix::Matrix::default());
    image::image_init_with_stream(&mut img, image_config);
    processing::seam_carve(img, desired_width, desired_height);
    // let mut handle = io::stdout().lock();
    // image::image_print(&mut img, &mut handle);
    let mut file = File::create(output_file).unwrap();
    image::image_print(&mut img, &mut file);

}

