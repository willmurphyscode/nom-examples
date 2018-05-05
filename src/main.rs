#[macro_use]
extern crate nom;

mod print_bits;
mod png_demo;

fn main() {
    println!("Demoing print bits");
    print_bits::demo();

}

