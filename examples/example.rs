#[macro_use(compose)] extern crate compose;
use std::ops::Mul;

fn rev<T>(mut v: Vec<T>) -> Vec<T> { v.reverse(); v }
fn sort<T: Ord>(mut v: Vec<T>) -> Vec<T> { v.sort(); v }
fn square<T: Copy + Mul<T, Output=T>>(mut v: Vec<T>) -> Vec<T> {
    v.into_iter().map(|e| (e * e) ).collect::<Vec<_>>()
}

fn main() {
    let v = vec![1, 10, 3, 16, 43, 2, 19];
    let desort = compose!( rev . sort . square );
    println!("{:?}", desort(v));
}
