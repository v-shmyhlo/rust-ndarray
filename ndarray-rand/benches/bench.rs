#![feature(test)]

extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
extern crate test;

use rand::distributions::Normal;
use ndarray::Array;
use ndarray_rand::RandomExt;
use ndarray_rand::F32;

use test::Bencher;

#[bench]
fn norm_f32(b: &mut Bencher) {
    let m = 100;
    b.iter(|| {
        let a: Array<f32, _> = Array::random((m, m), F32(Normal::new(0., 1.)));
        a
    });
}

#[bench]
fn norm_f64(b: &mut Bencher) {
    let m = 100;
    b.iter(|| {
        let a: Array<f64, _> = Array::random((m, m), Normal::new(0., 1.));
        a
    });
}
