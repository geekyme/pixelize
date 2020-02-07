extern crate clap;
extern crate image;
extern crate pixelize;
use clap::{App, Arg};

use std::fs;

const PARAM_IN: &str = "in";
const PARAM_OUT: &str = "out";
const PARAM_DEGREE: &str = "degree";

fn main() {
    let matches = App::new("pixelate")
        .version("1.0")
        .about("Pixellate supplied images")
        .author("Shawn Lim")
        .arg(
            Arg::with_name(PARAM_IN)
                .short("i")
                .long(PARAM_IN)
                .value_name("FILE")
                .help("Path to input image file")
                .required(true),
        )
        .arg(
            Arg::with_name(PARAM_OUT)
                .short("o")
                .long(PARAM_OUT)
                .value_name("FILE")
                .help("Path to output image file")
                .required(true),
        )
        .arg(
            Arg::with_name(PARAM_DEGREE)
                .short("d")
                .long(PARAM_DEGREE)
                .value_name("NUMBER")
                .help("Sets degree of pixelation. The higher this number, the more pixelated your image will be")
                .required(true),
        )
        .get_matches();

    let in_file = matches.value_of(PARAM_IN).unwrap();
    let out_file = matches.value_of(PARAM_OUT).unwrap();
    let d = matches
        .value_of(PARAM_DEGREE)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let buf = fs::read(in_file).unwrap();

    let out = pixelize::pixelize(buf.as_slice(), d).unwrap();

    match out.save(out_file) {
        Ok(_) => println!(
            "{} pixelated with degree {} and saved to {}",
            in_file, d, out_file
        ),
        Err(e) => panic!("Error pixelating your image: {}", e),
    }
}
