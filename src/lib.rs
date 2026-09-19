#![no_std]
#![allow(dead_code)]

mod linear_map;
mod ring_buffer;
mod string;
mod vec;

pub use ring_buffer::RingBuffer;
pub use vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Full,
}
