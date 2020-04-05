use image::{imageops, FilterType, GenericImageView, ImageBuffer, ImageError, RgbaImage};
use std::io::Cursor;

pub struct Options {
  pub degree: u32,
  pub preserve_size: bool,
}

pub fn pixelize(buf: &[u8], opts: &Options) -> Result<RgbaImage, ImageError> {
  let format = image::guess_format(buf)?;
  let c = Cursor::new(buf);
  let img = image::load(c, format).unwrap();
  let (w, h) = img.dimensions();
  if opts.degree >= w || opts.degree >= h {
    return Err(ImageError::UnsupportedError(String::from(
      "degree cannot exceed the dimensions of the image!",
    )));
  }

  if opts.degree <= 1 {
    return Err(ImageError::UnsupportedError(String::from(
      "degree needs to be > 1!",
    )));
  }

  let mut out: RgbaImage = ImageBuffer::new(w / opts.degree, h / opts.degree);
  let (new_w, new_h) = out.dimensions();
  for x in 0..new_w {
    for y in 0..new_h {
      let p = img.get_pixel(x * opts.degree, y * opts.degree);
      out.put_pixel(x, y, p);
    }
  }

  if opts.preserve_size {
    out = imageops::resize(&out, w, h, FilterType::Nearest);
  }

  return Ok(out);
}
