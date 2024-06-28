use std::cmp;
pub const MAX_MATRIX_WIDTH: usize = 500;
pub const MAX_MATRIX_HEIGHT: usize = 500;


#[derive(Clone)]
pub struct Matrix {
    pub width: i32,
    pub height: i32,
    pub data: Vec<i32>
}

impl Default for Matrix {

    fn default() -> Matrix {
        Matrix{width: 0, height: 0, data: Vec::new()}
    }

}



pub fn matrix_init(mat: &mut Box<Matrix>, width: i32, height: i32) -> () {
    mat.width = width;
    mat.height = height;
    (mat.data).resize(width as usize * height as usize, 0);
}

pub fn matrix_print(mat: &Box<Matrix>) {
    for i in 0..matrix_height(mat) {
        for j in 0..matrix_width(mat) {
            print!("{} ", unsafe{*const_matrix_at(mat, i, j)});
            if j == matrix_width(mat) - 1 {
                println!();
            }
        }
    }
}

pub fn matrix_width(mat: &Box<Matrix>) -> i32 {
    unsafe {
        return (&*mat).width;
    }
}

pub fn matrix_height(mat: &Box<Matrix>) -> i32 {
    unsafe {
        return (&*mat).height;
    }
}

// pub fn matrix_row(mat: &Box<Matrix>, ptr: *const i32) -> i32 {
//     let mat_ptr = mat as * const i32;
//     unsafe {
//         let offset: isize = ptr.offset_from(mat_ptr);
//         // println!("offset between start and end is {}", offset);
//         (offset / matrix_width(mat) as isize) as i32
//     }
// }
// pub fn matrix_column(mat: &Box<Matrix>, ptr: *const i32) -> i32 {

//     let mat_ptr = mat as *const i32;
//     unsafe {
//         let offset: isize = ptr.offset_from(mat_ptr);
//         // println!("offset between start and end is {}", offset);
//         (offset % matrix_width(mat) as isize) as i32
//     }
// }

pub fn matrix_at(mat: &mut Box<Matrix>, row: i32, col: i32) -> *mut i32 {
    unsafe {
        // let mat_ref = &mut *mat;
        let index = (row as usize * matrix_width(mat) as usize) + col as usize;
        return mat.data.as_mut_ptr().add(index);
    }
}

pub fn matrix_fill(mat: &mut Box<Matrix>, value: i32) {

    for i in 0..matrix_height(mat) {
        for j in 0..matrix_width(mat) {
            unsafe { *matrix_at(mat, i, j) = value; }
        }
    }

}

pub fn matrix_fill_border(mat: &mut Box<Matrix>, value: i32) {

    for j in 0..matrix_width(mat) {
        unsafe{
            *matrix_at(mat, 0, j) = value;
            *matrix_at(mat, matrix_height(mat) - 1, j) = value;
        }
    }

    for i in 0..matrix_height(mat) {
        unsafe {
            *matrix_at(mat, i, 0) = value;
            *matrix_at(mat, i, matrix_width(mat) - 1) = value;

        }
    }
}

pub fn matrix_max(mat: &Box<Matrix>) -> i32 {

    unsafe {
        let mut bigboi = *const_matrix_at(&mat, 0, 0);

        for (_i, val) in (&*mat).data.iter().enumerate() {
            bigboi = cmp::max(bigboi, *val);
        }
        bigboi
    }


}

pub fn const_matrix_at(mat: &Box<Matrix>, row: i32, col: i32) -> *const i32 {
    unsafe {
        let index = (row as usize * matrix_width(mat) as usize) + col as usize;
        &(*mat).data[index] as *const i32
    }
}

pub fn matrix_column_of_min_value_in_row(mat: &Box<Matrix>, row: i32, col_start: i32, col_end: i32) -> i32{

    unsafe {
        let mut min_col = col_start;
        let mut min_value = *const_matrix_at(mat, row, col_start);

        for j in col_start..col_end {
            let curr = *const_matrix_at(mat, row, j);
            if  curr < min_value {
                min_value = curr;
                min_col = j;
            }
        }
        min_col
    }
}

pub fn matrix_min_value_in_row(mat: &Box<Matrix>, row: i32) -> i32 {
    unsafe {
        let mut min_val = *const_matrix_at(mat, row, 0);
        for j in 1..matrix_width(mat) {
            let curr = *const_matrix_at(mat, row, j);
            min_val = cmp::min(min_val, curr);
        }

        min_val
    }

}

fn main() {
    // let mut _matrix = Matrix {height: 5, width: 4, data: [0; MAX_MATRIX_WIDTH * MAX_MATRIX_HEIGHT]};
    // matrix_print(&_matrix);
    // assert_eq!(unsafe{*matrix_at(&mut _matrix, 2, 2)}, 0);
    // println!("{}", matrix_column_of_min_value_in_row(&_matrix, 0, 0, 2));
    // matrix_fill_border(&mut _matrix, 7);
    // matrix_print(&_matrix);
    // println!("{}", matrix_column_of_min_value_in_row(&_matrix, 1, 1, 3));
    // println!("{}", matrix_column_of_min_value_in_row(&_matrix, 0, 2, 4));

    // let min1 = matrix_min_value_in_row(&_matrix, 1);
    // let min0 = matrix_min_value_in_row(&_matrix, 0);
    // println!("Minimum in Row 0 is {}", min0);
    // println!("Minimum in Row 1 is {}", min1);

}
