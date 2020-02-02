extern crate clap;
extern crate image;
use clap::{App, Arg};

use image::{imageops, FilterType, GenericImageView, ImageBuffer, RgbaImage};

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
    let d_u32 = matches
        .value_of(PARAM_DEGREE)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let img = image::open(in_file).unwrap();
    let (w, h) = img.dimensions();

    if d_u32 >= w || d_u32 >= h {
        eprintln!("degree cannot exceed the dimensions of the image!");
        std::process::exit(1);
    }

    let mut out: RgbaImage = ImageBuffer::new(w / d_u32, h / d_u32);
    let (new_w, new_h) = out.dimensions();

    for x in 0..new_w {
        for y in 0..new_h {
            let p = img.get_pixel(x * d_u32, y * d_u32);
            out.put_pixel(x, y, p);
        }
    }

    out = imageops::resize(&out, w, h, FilterType::Nearest);

    match out.save(out_file) {
        Ok(_) => println!(
            "{} pixelated with degree {} and saved to {}",
            in_file, d_u32, out_file
        ),
        Err(e) => eprintln!("Error pixelating your image: {}", e),
    }
}
