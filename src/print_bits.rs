use nom::IResult;


pub fn demo() {
    println!("Demoing a parser that pulls apart bits");
    let bytes = vec![
        0x00u8,
        0xFFu8,
        0x0Fu8,
    ];
    print_bits(&bytes[..]);
}

fn print_bits(bytes: &[u8]) {
    let mut print_me : Vec<String> = Vec::new();

    let mut slice = bytes; 
    loop {
        let result = take_8_bits(slice);
        if let IResult::Done(rest, string) = result {
            print_me.push(string);
            slice = rest;
            if slice.len() < 1 { break; }
        }
    }

    print_me.iter().for_each(|string| println!("{}", string));
}

named!(take_8_bits<&[u8], String>,
    bits!(
        do_parse!(
            first: take_bits!(u8, 1) >>
            second: take_bits!(u8, 1) >>
            third: take_bits!(u8, 1) >>
            fourth: take_bits!(u8, 1) >>
            fifth: take_bits!(u8, 1) >>
            sixth: take_bits!(u8, 1) >>
            seventh: take_bits!(u8, 1) >>
            eighth: take_bits!(u8, 1) >>
            (
                format!("{}, {}, {}, {}, {}, {}, {}, {}", first,
                 second, third, fourth, fifth, sixth, seventh, eighth)
            )
        )
    )
);