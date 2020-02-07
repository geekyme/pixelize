# Pixelize [![crates.io](https://img.shields.io/crates/v/pixelize.svg)](https://crates.io/crates/pixelize)

This crate provides:

- a CLI tool to pixelate your supplied image
- a utility `pixelize` function to do the same thing

## CLI Installation

```
cargo install pixelize
```

## CLI Usage

```
pixelize -i girl.jpg -o girl_10.jpg -d 10
```

| Original              | -d 10                         |
| --------------------- | ----------------------------- |
| ![Before](./girl.jpg) | ![After @ 10d](./girl_10.jpg) |

| -d 20                         | -d 50                       |
| ----------------------------- | --------------------------- |
| ![After @ 20d](./girl_20.jpg) | ![After 50d](./girl_50.jpg) |

## Lib Installation

```
[dependencies]
pixelize = "0.2.0"
```

## Lib Usage

```
let d = 10;
let in_file = "temp.jpg";
let out_file = "temp_10.jpg";
let buf = fs::read(in_file).unwrap();

let out = pixelize::pixelize(buf.as_slice(), d).unwrap();

match out.save(out_file) {
    Ok(_) => println!(
        "{} pixelated with degree {} and saved to {}",
        in_file, d, out_file
    ),
    Err(e) => panic!("Error pixelating your image: {}", e),
}
```
