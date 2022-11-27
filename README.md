# wallpapergen

A CLI tool for generating gradient wallpapers.

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

![Wallpaper sample 1](./examples/1.png)
![Wallpaper sample 2](./examples/2.png)
![Wallpaper sample 3](./examples/3.png)
![Wallpaper sample 4](./examples/4.png)
![Wallpaper sample 5](./examples/5.png)



[rust]: <https://www.rust-lang.org/tools/install>
[catppuccin]: <https://github.com/catppuccin/catppuccin>
