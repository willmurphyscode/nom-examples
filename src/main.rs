#[macro_use]
extern crate nom;

mod parse_cargo;
mod print_bits;
mod png_demo;


fn main() {
    parse_cargo::demo();
//    print_bits::demo();
//    png_demo::demo();
}

