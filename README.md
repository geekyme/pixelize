# Pixelize [![crates.io](https://img.shields.io/crates/v/[pixelize].svg)](https://crates.io/crates/pixelize)

This crate provides a CLI to pixelate your supplied image.

## Installation

```
cargo install pixelize
```

## Example

```
pixelize -i girl.jpg -o girl_10.jpg -d 10
```

Before:

![Before](./girl.jpg)

After (10d, 20d, 50d):

![After](./girl_10.jpg)

![After](./girl_20.jpg)

![After](./girl_50.jpg)
