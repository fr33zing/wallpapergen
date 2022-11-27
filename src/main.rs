// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

use clap::Parser;
use image::{save_buffer_with_format, ColorType, ImageFormat};
use rand::{seq::SliceRandom, Rng};
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'W', long)]
    width: u32,

    #[arg(short = 'H', long)]
    height: u32,

    #[arg(short, long = "color")]
    colors: Vec<csscolorparser::Color>,
}

#[derive(Clone)]
struct ColorPoint {
    u: f64,
    v: f64,
    channels: [f64; 3],
}

fn rng_color_points(min: usize, max: usize, colors: Vec<csscolorparser::Color>) -> Vec<ColorPoint> {
    let mut rng = rand::thread_rng();
    (0..(rng.gen_range(min..max)))
        .map(|_| {
            let color = colors.choose(&mut rng).expect("failed to choose color");
            ColorPoint {
                u: rng.gen(),
                v: rng.gen(),
                channels: [color.r, color.g, color.b],
            }
        })
        .collect()
}

fn distance(u: f64, v: f64, uu: f64, vv: f64) -> f64 {
    f64::sqrt(f64::powi(uu - u, 2) + f64::powi(vv - v, 2))
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

const GRADIENT_MOD: f64 = 1.0;
const MAX_POINTS: usize = 8;

fn main() -> Result<(), image::ImageError> {
    let args = Args::parse();
    let w = args.width as f64;
    let h = args.height as f64;
    let points_max = usize::min(MAX_POINTS, args.colors.len());
    let points = rng_color_points(2, points_max, args.colors);
    let base_color = points
        .choose(&mut rand::thread_rng())
        .expect("failed to choose base color")
        .channels;
    let size = args.width * args.height * 3;
    let pixels: Vec<u8> = (0..size)
        .into_par_iter()
        .map_with(points, |points, i| {
            let x = (i / 3) / args.width;
            let y = (i / 3) % args.width;
            let u: f64 = (x as f64) / h;
            let v: f64 = (y as f64) / w;
            let channel: usize = (i % 3).try_into().unwrap();

            let value = points
                .into_iter()
                .map(|p| {
                    let dist = distance(u, v, p.u, p.v);
                    let color = p.channels[channel];
                    (color, dist)
                })
                .fold(base_color[channel], |acc, val| {
                    lerp(acc, val.0, (1.0 - val.1) * GRADIENT_MOD)
                });

            (value * 255.0) as u8
        })
        .collect();

    save_buffer_with_format(
        "test.png",
        &pixels,
        args.width,
        args.height,
        ColorType::Rgb8,
        ImageFormat::Png,
    )
}
