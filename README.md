# zero

> Zero, `static` memory allocated data structures. Inspired based on [Heapless](https://github.com/rust-embedded/heapless)


### List of implemented data structures
- [`Vec`]: A vector.
- [`String`]: A string.
- [`SPSC`]: A lock-free single-producer, single-consumer queue.
- [`LinearMap`]: A linear map, a key-value data structure. 


### Acknowledgments & Learning Path

This project is a personal learning effort focused on implementing data structures with static memory allocation in Rust. 

The design and API patterns were heavily inspired by [heapless](https://github.com/rust-embedded/heapless). While this project re-implements similar concepts, it was developed independently as a way to understand the underlying mechanics of `no_std` Rust, atomic synchronization, and memory safety.

*This project is not affiliated with or endorsed by the authors of `heapless`.*