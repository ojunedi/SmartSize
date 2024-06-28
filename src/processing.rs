use crate::{matrix, image};
use crate::matrix::Matrix;
use crate::image::{Image, Pixel};
use std::cmp;
use std::alloc::{dealloc, alloc, Layout};
use std::io;

pub fn squared_difference(p1: &Pixel, p2: &Pixel) -> i32 {
    let dr = p2.r - p1.r;
    let dg = p2.g - p1.g;
    let db = p2.b - p1.b;

    (dr*dr + dg*dg + db*db) / 100
}

pub fn rotate_left(img: &mut Box<Image>) {

    let width = image::image_width(img);
    let height = image::image_height(img);

    let mut aux: Box<Image> = Box::new(Image {
        width: width,
        height: height,
        red_channel: Box::new(matrix::Matrix::default()),
        blue_channel: Box::new(matrix::Matrix::default()),
        green_channel: Box::new(matrix::Matrix::default())
    });
    image::image_init(&mut aux, height, width);

    for r in 0..height {
        for c in 0..width {
            image::image_set_pixel(&mut aux, width - c - 1, r, &image::image_get_pixel(img, r, c));
        }
    }
    // image::image_print(&mut aux, &mut io::stdout().lock());

    *img = aux;

}

pub fn rotate_right(img: &mut Box<Image>) {

    let width = image::image_width(img);
    let height = image::image_height(img);

    let mut aux: Box<Image> = Box::new(Image {
        width: width,
        height: height,
        red_channel: Box::new(matrix::Matrix::default()),
        blue_channel: Box::new(matrix::Matrix::default()),
        green_channel: Box::new(matrix::Matrix::default()),
    });
    image::image_init(&mut aux, height, width);

    for r in 0..height {
        for c in 0..width {
            image::image_set_pixel(&mut aux, c, height - r - 1, &image::image_get_pixel(img, r, c));
        }
    }


    *img = aux;
}

pub fn compute_energy_matrix(img: &mut Box<Image>, energy: &mut Box<Matrix>) {

    let width = image::image_width(img);
    let height = image::image_height(img);
    matrix::matrix_init(energy, width, height);
    matrix::matrix_fill(energy, 0);

    for row in 1..height-1 {
        for col in 1..width-1 {
            let N = &image::image_get_pixel(img, row - 1, col);
            let S = &image::image_get_pixel(img, row + 1, col);
            let E = &image::image_get_pixel(img, row, col + 1);
            let W = &image::image_get_pixel(img, row, col - 1);
            // println!("N is {} {} {}", N.r, N.g, N.b);
            // println!("S is {} {} {}", S.r, S.g, S.b);
            // println!("E is {} {} {}", E.r, E.g, E.b);
            // println!("W is {} {} {}", W.r, W.g, W.b);
            // println!("ns {}", squared_difference(N, S));
            // println!("ew {}", squared_difference(E, W));
            unsafe { *matrix::matrix_at(energy, row, col) = squared_difference(N, S) + squared_difference(E, W); }
        }
    }

    matrix::matrix_fill_border(energy, matrix::matrix_max(energy));
}

pub fn compute_vertical_cost_matrix(energy: &Box<Matrix>, cost: &mut Box<Matrix>) {

    let width = matrix::matrix_width(energy);
    let height = matrix::matrix_height(energy);
        // println!("{} {}", width, height);
        matrix::matrix_init(cost, width, height);
        for col in 0..width {
            unsafe {*matrix::matrix_at(cost, 0, col) = *matrix::const_matrix_at(energy, 0, col); }
        }
        for row in 1..height {
            for col in 0..width {
                    let mut min_col: i32 = 0;
                    if col == 0 {
                        min_col = matrix::matrix_column_of_min_value_in_row(cost, row - 1, 0, 2);
                    } else if col == width - 1 {
                        min_col = matrix::matrix_column_of_min_value_in_row(cost, row - 1 , col - 1, col + 1);
                    } else  {
                        min_col = matrix::matrix_column_of_min_value_in_row(cost, row - 1, col - 1, col + 2);
                    }
                unsafe {
                    *matrix::matrix_at(cost, row, col) = *matrix::const_matrix_at(energy, row, col) + *matrix::const_matrix_at(cost, row - 1, min_col);
                }
            }
        }
    }

pub fn find_minimal_vertial_seam(cost: &Box<Matrix>, seam: &mut [i32]) {

    let height = matrix::matrix_height(cost);
    let width = matrix::matrix_width(cost);
    seam[(height - 1) as usize] = matrix::matrix_column_of_min_value_in_row(cost, height - 1, 0, width);
    // println!("{}", seam[(height - 1) as usize]);

    for row in (0..height-1).rev() {
        let r = row as usize;
        if seam[r + 1] == 0 {
            seam[r] = matrix::matrix_column_of_min_value_in_row(cost, row, 0, 2);
            // println!("seam at 0");
        } else if seam[r + 1] == width - 1 {
            seam[r] = matrix::matrix_column_of_min_value_in_row(cost, row, seam[r + 1] - 1, seam[r + 1] + 1);
            // println!("seam at end of row");
        } else {
            seam[r] = matrix::matrix_column_of_min_value_in_row(cost, row, seam[r + 1] - 1, seam[r + 1] + 2);
            // println!("seam somewhere in the middle");
        }
    }

    // for i in (0..height) { println!("{}", seam[i as usize]); }
}

pub fn remove_vertial_seam(img: &mut Box<Image>, seam: &mut [i32]) {

    let width = image::image_width(img);
    let height = image::image_height(img);

    let layout = Layout::new::<image::Image>();
    unsafe {
        let mut aux: Box<Image> = Box::new(Image {
            width: width - 1,
            height,
            red_channel: Box::new(matrix::Matrix::default()),
            blue_channel: Box::new(matrix::Matrix::default()),
            green_channel: Box::new(matrix::Matrix::default()),
        });
        image::image_init(&mut aux, width - 1, height);

        for row in 0..height {
            for col in (0..width - 1) {
                if col < seam[row as usize] {
                    let pixel = &image::image_get_pixel(img, row, col);
                    // println!("less than {} {}", row, col);
                    // println!("{} {} {}", pixel.r, pixel.g, pixel.b);
                    image::image_set_pixel(&mut aux, row, col, pixel);
                } else {
                    let pixel = &image::image_get_pixel(img, row, col + 1);
                    // println!("greater than or equal to {} {} seam {}", row, col, seam[row as usize]);
                    // println!("{} {} {}", pixel.r, pixel.g, pixel.b);
                    image::image_set_pixel(&mut aux, row, col, pixel);
                }
            }
        }
        // image::image_print(aux);
        *img = aux;
    }
}

pub fn seam_carve_width(img: &mut Box<Image>, newWidth: i32) {

    let layout = Layout::new::<Matrix>();
    let mut energy: Box<matrix::Matrix> = Box::new(matrix::Matrix::default());
    let mut cost  : Box<matrix::Matrix> = Box::new(matrix::Matrix::default());
    let mut seam =[0; matrix::MAX_MATRIX_HEIGHT];

    while (image::image_width(img) > newWidth) {
        compute_energy_matrix(img, &mut energy);
        compute_vertical_cost_matrix(&mut energy, &mut cost);
        find_minimal_vertial_seam(&mut cost, &mut seam);
        remove_vertial_seam(img, &mut seam);
    }

}

pub fn seam_carve_height(img: &mut Box<Image>, newHeight: i32) {

    rotate_left(img);
    seam_carve_width(img, newHeight);
    rotate_right(img);
}

pub fn seam_carve(img: &mut Box<Image>, newWidth: i32, newHeight: i32) {

    seam_carve_width(img, newWidth);
    seam_carve_height(img, newHeight);
    // image::image_print(img, &mut io::stdout().lock());

}
