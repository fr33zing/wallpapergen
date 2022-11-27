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

//! wallpapergen - gradient wallpaper generator

use std::{env, path::PathBuf};

use clap::Parser;
use rand::prelude::*;
use rayon::prelude::*;

const GRADIENT_FACTOR: f64 = 1.0;
const MAX_POINTS: usize = 10;
const MIN_POINTS: usize = 3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, max_term_width = 80)]
struct Args {
    /// Output image width.
    #[arg(short = 'W', long)]
    width: u32,

    /// Output image height.
    #[arg(short = 'H', long)]
    height: u32,

    /// Wallpaper colors.
    ///
    /// Adds one CSS color to the list of available colors. Provide this
    /// argument multiple times to add more colors.
    #[arg(short, long = "color", value_name = "COLOR", required = true)]
    colors: Vec<csscolorparser::Color>,

    /// (Optional) Output file path.
    ///
    /// Image format is determined by file extension. If this argument is not
    /// provided, a timestamped filename will be generated. Output directory
    /// defaults to the current working directory.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// (Optional) Random number generator seed.
    ///
    /// Specifies a 64 bit integer to seed the SmallRng.
    #[arg(short, long, value_name = "INT")]
    seed: Option<u64>,
}

#[derive(Clone)]
struct ColorPoint {
    u: f64,
    v: f64,
    channels: [f64; 3],
}

fn rng_color_points(
    rng: &mut impl rand::Rng,
    max: usize,
    colors: Vec<csscolorparser::Color>,
) -> Vec<ColorPoint> {
    let min = usize::min(colors.len(), MIN_POINTS);
    (0..(rng.gen_range(min..max)))
        .map(|_| {
            let color = colors.choose(rng).expect("failed to choose color");
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

fn new_rng(seed: Option<u64>) -> SmallRng {
    match seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_rng(thread_rng()).expect("failed to create RNG"),
    }
}

fn timestamp_filename() -> PathBuf {
    let now = std::time::SystemTime::now();
    let now: chrono::DateTime<chrono::Utc> = now.into();
    let filename = format!("wallpapergen_{}.png", now.to_rfc3339());
    let cwd = env::current_dir().expect("failed to get current working directory");
    cwd.join(&filename)
}

fn main() {
    let args = Args::parse();
    let mut rng = new_rng(args.seed);
    let w = args.width as f64;
    let h = args.height as f64;
    let points = rng_color_points(&mut rng, MAX_POINTS, args.colors);
    let base_color = match points.choose(&mut rng) {
        Some(p) => p.channels,
        None => [0.0, 0.0, 0.0],
    };
    let size = args.width * args.height * 3;
    let pixels: Vec<u8> = (0..size)
        .into_par_iter()
        .map_with(points, |points, i| {
            let x = (i / 3) % args.width;
            let y = (i / 3) / args.width;
            let u: f64 = (x as f64) / w;
            let v: f64 = (y as f64) / h;
            let channel: usize = (i % 3).try_into().unwrap();
            let value = points
                .into_iter()
                .map(|p| {
                    let dist = distance(u, v, p.u, p.v);
                    let color = p.channels[channel];
                    (color, dist)
                })
                .fold(base_color[channel], |acc, val| {
                    lerp(acc, val.0, (1.0 - val.1) * GRADIENT_FACTOR)
                });

            (value * 255.0) as u8
        })
        .collect();

    let output = args.output.unwrap_or_else(|| timestamp_filename());
    image::save_buffer(
        output.clone(),
        &pixels,
        args.width,
        args.height,
        image::ColorType::Rgb8,
    )
    .expect("failed to write image file");
    println!("{}", output.display());
}
