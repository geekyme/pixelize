extern crate image;

use image::{imageops, FilterType, GenericImageView, ImageBuffer, ImageError, RgbaImage};
use std::io::Cursor;

pub fn pixelize(buf: &[u8], d: u32) -> Result<RgbaImage, ImageError> {
    let format = image::guess_format(buf)?;
    let c = Cursor::new(buf);
    let img = image::load(c, format).unwrap();
    let (w, h) = img.dimensions();
    if d >= w || d >= h {
        return Err(ImageError::UnsupportedError(String::from(
            "degree cannot exceed the dimensions of the image!",
        )));
    }

    if d <= 1 {
        return Err(ImageError::UnsupportedError(String::from(
            "degree needs to be > 1!",
        )));
    }

    let mut out: RgbaImage = ImageBuffer::new(w / d, h / d);
    let (new_w, new_h) = out.dimensions();
    for x in 0..new_w {
        for y in 0..new_h {
            let p = img.get_pixel(x * d, y * d);
            out.put_pixel(x, y, p);
        }
    }

    out = imageops::resize(&out, w, h, FilterType::Nearest);

    return Ok(out);
}
