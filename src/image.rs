use crate::matrix;
use crate::Config;
use std::fs;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::io::Write;
use std::mem;

pub struct Pixel {
     pub r: i32,
     pub g: i32,
     pub b: i32
}

const MAX_INTENSITY: i32 = 255;

#[derive(Clone)]
pub struct Image {
    pub width: i32,
   pub height: i32,
    pub red_channel: Box<matrix::Matrix>,
    pub green_channel: Box<matrix::Matrix>,
    pub blue_channel: Box<matrix::Matrix>
}

impl Default for Image {
    fn default() -> Image {
        Image {
            height: 0,
            width: 0,
                red_channel: Box::new(matrix::Matrix::default()),
                green_channel: Box::new(matrix::Matrix::default()),
                blue_channel: Box::new(matrix::Matrix::default())
            }
        }
    }

    pub fn image_init(img: &mut Box<Image>, width: i32, height: i32) {

        unsafe {
            (*img).width = width;
            (*img).height = height;

            matrix::matrix_init(&mut img.red_channel, width, height);
            matrix::matrix_init(&mut img.green_channel, width, height);
            matrix::matrix_init(&mut img.blue_channel, width, height);

        }
    }

    pub fn image_init_with_stream(img: &mut Box<Image>, image_config: Config) -> io::Result<()>{

    let content = fs::read_to_string(image_config.input_file)?;
    let rgb_values: Vec<&str> = content.split_whitespace().collect();
    let width = rgb_values[1].parse::<i32>().unwrap();
    let height = rgb_values[2].parse::<i32>().unwrap();

    img.width = width;
    img.height = height;

    matrix::matrix_init(&mut img.red_channel, width, height);
    matrix::matrix_init(&mut img.green_channel, width, height);
    matrix::matrix_init(&mut img.blue_channel, width, height);


    let mut index: usize = 0;
    for i in (4..rgb_values.len() - 2).step_by(3) {
        unsafe {
            (*img).red_channel.data[index] = rgb_values[i].parse::<i32>().unwrap();
            (*img).green_channel.data[index] = rgb_values[i + 1].parse::<i32>().unwrap();
            (*img).blue_channel.data[index] = rgb_values[i + 2].parse::<i32>().unwrap();

        }
        index += 1;
    }
    //println!();
    //unsafe {
    //    matrix::matrix_print(&(*img).red_channel);
    //    matrix::matrix_print(&(*img).green_channel);
    //    matrix::matrix_print(&(*img).blue_channel);
    //}
    Ok(())

}

pub fn image_print<W: Write>(img:  &mut Box::<Image>, writer: &mut W) -> std::io::Result<()> {
    // println!("P3");
    // println!("{}", img.red_channel.data.len());
    // println!("{}", img.green_channel.data.len());
    // println!("{}", img.blue_channel.data.len());
    writer.write_all(b"P3\n")?;
    let height_width = format!("{} {}\n", image_width(img), image_height(img));
    writer.write_all(height_width.as_bytes());
    // println!("{} {}", image_width(img), image_height(img));
    writer.write_all(b"255\n");
    for row in 0..image_height(img) {
       for col in 0..image_width(img) {
           let p = image_get_pixel(img, row, col);
           let line = format!("{} {} {} ", p.r, p.g, p.b);
           writer.write_all(line.as_bytes());
       }
       writer.write_all(b"\n");
    }
    Ok(())
}

pub fn image_width(img: &Box<Image>) -> i32 {
    unsafe {
        (*img).width
    }
}

pub fn image_height(img: &Box<Image>)-> i32 {
    unsafe {
        (*img).height
    }
}

pub fn image_get_pixel(img: &mut Box<Image>, row: i32, column: i32) -> Pixel {
    unsafe {
        let p = Pixel {
            r: (*matrix::const_matrix_at(&mut img.red_channel, row, column)),
            g: (*matrix::const_matrix_at(&mut img.green_channel, row, column)),
            b: (*matrix::const_matrix_at(&mut img.blue_channel, row, column))
        };
        p
    }
}

pub fn image_set_pixel(img: &mut Box<Image>, row: i32, column: i32, color: &Pixel) -> () {

    unsafe {
        *matrix::matrix_at((&mut img.red_channel), row, column) = color.r;
        *matrix::matrix_at((&mut img.green_channel), row, column) = color.g;
        *matrix::matrix_at((&mut img.blue_channel), row, column) = color.b;
    }

}

pub fn image_fill(img: &mut Box<Image>, color: &Pixel) {

    for i in 0..image_width(img) {
        for j in 0..image_height(img) {
            image_set_pixel(img, i, j, color);
        }
    }

}



pub fn run() {
    let mut img = Image::default();
    println!("width is {} and height is {}", img.width, img.height);
    // matrix::matrix_fill_border(&mut img.green_channel, 8);
    // matrix::matrix_print(&img.green_channel);
    // matrix::matrix_print(&img.blue_channel);
    // matrix::matrix_print(&img.red_channel);
}
