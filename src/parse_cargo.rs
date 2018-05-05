use nom;

pub fn demo() {
    println!("Demoing a parser that matches /[Cc]argo/:");
    println!();
    print_parse_results("cargo");
    print_parse_results("car");
    print_parse_results("some other string");
    print_parse_results("Cargo + more");
    print_parse_results("Some stuff before 'cargo'");
}

fn print_parse_results(input: &str) {
    print!("The parse result for '{}' is ", input);
    match parse_cargo(input.as_bytes()) {
        nom::IResult::Done(input, output) => println!("'Done', rest is {:?} and output is {:?}", input, output),
        nom::IResult::Error(err) => println!("Error: {:?}", err),
        nom::IResult::Incomplete(needed) => println!("Incompled, needed {:?}", needed),
    }
}

named!(parse_cargo,
    alt!(tag!(&b"cargo"[..]) | tag!(&b"Cargo"[..]))
);
