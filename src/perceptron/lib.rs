#[macro_use]
extern crate rulinalg;

use rulinalg::matrix::{BaseMatrix, Matrix};

fn and_gate(x1: f32, x2: f32) -> f32 {
    let x = matrix![x1, x2];
    let w = matrix![0.5, 0.5f32];
    let b = -0.7;
    let out = x.elemul(&w).sum() + b;

    if out > 0f32 {
        1.0
    } else {
        0.0
    }
}

fn main() {
    let input: Matrix<f32> = matrix![
        0.0, 0.0; 
        0.0, 1.0; 
        1.0, 0.0; 
        1.0, 1.0];

    for x in input.row_iter() {
        let mut row = x.iter();
        let x1 = row.next().unwrap();
        let x2 = row.next().unwrap();

        println!("({}, {}) -> {}", x1, x2, and_gate(*x1, *x2));
    }
}
