# Pixelize [![crates.io](https://img.shields.io/crates/v/[pixelize].svg)](https://crates.io/crates/pixelize)

This crate provides a CLI tool to pixelate your supplied image.

## Installation

```
cargo install pixelize
```

## Usage

```
pixelize -i girl.jpg -o girl_10.jpg -d 10
```

| Original              | -d 10                         |
| --------------------- | ----------------------------- |
| ![Before](./girl.jpg) | ![After @ 10d](./girl_10.jpg) |

| -d 20                         | -d 50                       |
| ----------------------------- | --------------------------- |
| ![After @ 20d](./girl_20.jpg) | ![After 50d](./girl_50.jpg) |
