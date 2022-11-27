# wallpapergen

![Crates.io](https://img.shields.io/crates/v/wallpapergen)
![Crates.io](https://img.shields.io/crates/l/wallpapergen)

A CLI tool for generating gradient wallpapers.

([View samples](#samples))

## Installation

If you don't have Rust, follow the installation instructions [here][rust].

Run the following command to install wallpapergen:

```sh
cargo install wallpapergen
```

## Usage

Run the following command to view help:

```sh
wallpapergen --help
```

Here is the command used to generate the samples seen below:

```sh
wallpapergen -W 825 -H 350 \
    -c '#f2cdcd' \
    -c '#f5c2e7' \
    -c '#cba6f7' \
    -c '#f38ba8' \
    -c '#eba0ac' \
    -c '#fab387' \
    -c '#f9e2af' \
    -c '#a6e3a1' \
    -c '#94e2d5' \
    -c '#89dceb' \
    -c '#74c7ec' \
    -c '#89b4fa' \
    -c '#b4befe'
```

(These colors are from [catppuccin][catppuccin])

## Samples

![Wallpaper sample 1](https://github.com/fr33zing/wallpapergen/blob/main/examples/1.png?raw=true)
![Wallpaper sample 2](https://github.com/fr33zing/wallpapergen/blob/main/examples/2.png?raw=true)
![Wallpaper sample 3](https://github.com/fr33zing/wallpapergen/blob/main/examples/3.png?raw=true)
![Wallpaper sample 4](https://github.com/fr33zing/wallpapergen/blob/main/examples/4.png?raw=true)


[rust]: <https://www.rust-lang.org/tools/install>
[catppuccin]: <https://github.com/catppuccin/catppuccin>
