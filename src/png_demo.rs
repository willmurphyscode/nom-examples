use nom;
use nom::IResult;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;


pub fn demo() {
    let f : File = File::open("png_example.png").expect("Could not open example file");
    let mut reader = BufReader::new(f);
    let mut bytes : Vec<u8> = vec![];
    let _ = reader.read(&mut bytes).expect("Could not read file"); 
    match png_header(&bytes[..]) {
        nom::IResult::Error(_) => println!("Could not parse file"),
        nom::IResult::Done(_, header_result) => {

        },
        _ => unimplemented!()
    }
}

#[derive(Debug)]
struct PngHeader {
    width: i32, // the field in PNGs has the same max as i32 because not every language supports u32
    height: i32,
    bit_depth: u8,
    color_type: ColorType,
    filter_method: u8,
    interlace_method: u8,
}

#[derive(Debug)]
enum ColorType {
    Grayscale,
    RGBTriple,
    PaletteIndex,
    GrayscaleWithAlpha,
    RGBTripleWithAlpha,
}


fn parse_color_type(byte: u8) -> Result<ColorType, ()> {
    match byte {
        0 => Ok(ColorType::Grayscale),
        2 => Ok(ColorType::RGBTriple),
        3 => Ok(ColorType::PaletteIndex),
        4 => Ok(ColorType::GrayscaleWithAlpha),
        6 => Ok(ColorType::RGBTripleWithAlpha),
        _ => Err(()),
    }
}

named!(take_an_int(&[u8]) -> i32, do_parse!(a: i32!( nom::Endianness::Little) >> ( a )));



named!(png_header( &[u8] ) -> Result<PngHeader, ()>,
    do_parse!(
        width: i32!(nom::Endianness::Little) >>
        height: i32!(nom::Endianness::Little) >>
        bit_depth: take!(1) >>
        color_type_byte: take!(1) >>
        filter_method: take!(1) >>
        interlace_method: take!(1)>>
        (
            if let Ok(color_type) = parse_color_type(color_type_byte[0]) {
                Ok(
                    PngHeader {
                    width: width,
                    height: height,
                    bit_depth: bit_depth[0],
                    color_type: color_type,
                    filter_method: filter_method[0],
                    interlace_method: filter_method[0],
                })
            } else {
                Err(())
            }
        )
    )
);