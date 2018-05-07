use nom;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;


pub fn demo() {
    let f : File = File::open("png_example.png").expect("Could not open example file");
    let mut reader = BufReader::new(f);
    let mut bytes : Vec<u8> = vec![];
    let _ = reader.read_to_end(&mut bytes).expect("Could not read file"); 
    // let a : () = png_header;
    match png_header(&bytes[..]) {
        nom::IResult::Error(_) => println!("Could not parse file"),
        nom::IResult::Done(_, header_result) => println!("{:?}", header_result),
        nom::IResult::Incomplete(needed) => println!("Tried to parse, but needed {:?}", needed),
    }
}

#[derive(Debug)]
struct PngHeader {
    width: u32, // the field in PNGs has the same max as i32 because not every language supports u32
    height: u32,
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

// TODO: Can I do this with pure nom? 
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

named!(color_type_grayscale<&[u8], ColorType>, do_parse!(a: tag!(&[0][..]) >> (ColorType::Grayscale)));
named!(color_type_rgb_triple<&[u8], ColorType>, do_parse!(a: tag!(&[2][..]) >> (ColorType::RGBTriple)));
named!(color_type_palette_index<&[u8], ColorType>, do_parse!(a: tag!(&[3][..]) >> (ColorType::PaletteIndex)));
named!(color_type_grayscale_with_alpha<&[u8], ColorType>, do_parse!(a: tag!(&[4][..]) >> (ColorType::GrayscaleWithAlpha)));
named!(color_type_rgb_triple_with_alpha<&[u8], ColorType>, do_parse!(a: tag!(&[6][..]) >> (ColorType::RGBTripleWithAlpha)));

named!(color_type<&[u8], ColorType>, 
    alt!(color_type_grayscale 
        | color_type_rgb_triple 
        | color_type_palette_index
        | color_type_grayscale_with_alpha
        | color_type_rgb_triple_with_alpha )
);

named!(take_an_int(&[u8]) -> i32, do_parse!(a: i32!( nom::Endianness::Little) >> ( a )));

static PNG_FILE_SIGNATURE : [u8; 8] = [
    137, 80, 78, 71, 13, 10, 26, 10
];

named!(png_signature<&[u8], &[u8]>, tag!(&PNG_FILE_SIGNATURE[..]));

// makes a function called `png_header`
// with type
// for<'r> fn(&'r [u8]) -> nom::IResult<&'r [u8], png_demo::PngHeader>
named!(png_header( &[u8] ) -> PngHeader,
    do_parse!(
        _signature: png_signature >>
        _chunk_length: take!(4) >>
        _chunk_type: take!(4) >>
        width: u32!(nom::Endianness::Big) >>
        height: u32!(nom::Endianness::Big) >>
        bit_depth: take!(1) >>
        color_type: color_type >>
        filter_method: take!(1) >>
        interlace_method: take!(1)>>
        (
            PngHeader {
                width: width,
                height: height,
                bit_depth: bit_depth[0],
                color_type: color_type,
                filter_method: filter_method[0],
                interlace_method: filter_method[0],
            }
        )
    )
);