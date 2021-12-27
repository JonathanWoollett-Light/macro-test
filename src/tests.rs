#![allow(dead_code)]

use crate::{forward, forward_autodiff};

#[forward_autodiff]
fn empty(x: f32) -> f32 {
    return x;
}
#[forward_autodiff]
fn plus(x: f32) -> f32 {
    return x+1f32;
}
#[forward_autodiff]
fn quad(x: f32) -> f32 {
    let a = x.powi(2i32);
    let b = x * 2f32;
    let c = 2f32;
    let f = a + b + c;
    return f;
}

#[test]
fn empty_test() {
    let (x, der_x) = forward!(empty, 1f32, 1f32);
    assert_eq!(x, 1.);
    assert_eq!(der_x, 1.);
}
#[test]
fn plus_test() {
    let (x, der_x) = forward!(plus, 1f32, 1f32);
    assert_eq!(x, 2f32);
    assert_eq!(der_x, 1f32);
}
#[test]
fn quad_test() {
    let (x, der_x) = forward!(quad, 3f32, 1f32);
    assert_eq!(x, 17f32);
    assert_eq!(der_x, 8f32);
}