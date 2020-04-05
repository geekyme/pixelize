extern crate clap;
extern crate image;
extern crate pixelize;
use clap::{App, Arg};

use std::fs;

const PARAM_IN: &str = "in";
const PARAM_OUT: &str = "out";
const PARAM_DEGREE: &str = "degree";
const PARAM_PRESERVE_SIZE: &str = "preserve-size";

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
        .arg(
            Arg::with_name(PARAM_PRESERVE_SIZE)
                .long(PARAM_PRESERVE_SIZE)
                .value_name("BOOLEAN")
                .help("Preserve the output size of the image")
        )
        .get_matches();

  let in_file = matches.value_of(PARAM_IN).unwrap();
  let out_file = matches.value_of(PARAM_OUT).unwrap();

  let opts = pixelize::Options {
    degree: matches
      .value_of(PARAM_DEGREE)
      .unwrap()
      .parse::<u32>()
      .unwrap(),
    preserve_size: if matches.is_present(PARAM_PRESERVE_SIZE) {
      matches
        .value_of(PARAM_PRESERVE_SIZE)
        .unwrap()
        .parse::<bool>()
        .unwrap()
    } else {
      true
    },
  };
  let buf = fs::read(in_file).unwrap();

  let out = pixelize::pixelize(buf.as_slice(), &opts).unwrap();

  match out.save(out_file) {
    Ok(_) => println!(
      "{} pixelated with degree {} and saved to {}",
      in_file, &opts.degree, out_file
    ),
    Err(e) => panic!("Error pixelating your image: {}", e),
  }
}
