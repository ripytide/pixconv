//! A crate for converting pixels and images between pixels types and
//! their respective color-spaces.
#![allow(dead_code)]
#![allow(unused_variables)]

mod color;
mod matrix;
mod traits;

mod impls;


type Image<P> = Vec<P>;
