use clap::Parser;
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use imageproc::noise::gaussian_noise;
use tartan;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Output name
    #[clap(short = 'o', long, default_value = "tartan.png")]
    filename: String,

    /// Width of the produced image.
    #[clap(short, long, default_value_t = 400)]
    width: u32,

    /// Height of the produced image.
    #[clap(short, long, default_value_t = 400)]
    height: u32,

    /// Skip n colour spans in the pattern.
    #[clap(short, long, default_value_t = 0)]
    skip: usize,

    /// Faster render without noise and blur
    #[clap(short, long)]
    ugly: bool,

    /// Thread count or sett
    #[clap(default_value = "W6T36G20T4RB20T4RB20T4G20T36R6")]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let sett = tartan::parse(&args.pattern);

    let mut image: RgbImage = ImageBuffer::new(args.width, args.height);

    // warp
    for y in 0..args.height {
        for (x, colour) in (0..args.width).zip(sett.colours(args.skip)) {
            image.put_pixel(x, y, Rgb(colour.to_array()));
        }
    }

    // weft
    for x in 0..args.width {
        for (y, colour) in (0..args.height).zip(sett.colours(args.skip)) {
            if (x + y) / 2 % 2 == 0 {
                image.put_pixel(x, y, Rgb(colour.to_array()));
            }
        }
    }
    if !args.ugly {
        image = gaussian_noise(&image, 1.5, 4.5, 1);
        image = imageops::blur(&image, 0.7);
    }
    image.save(args.filename).unwrap();
}
