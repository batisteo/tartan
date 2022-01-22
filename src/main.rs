mod cli;

use crate::cli::Parameters;
use tartanlib as tartan;

use image::{imageops, ImageBuffer, Rgb, RgbImage};
use imageproc::noise::gaussian_noise;

fn main() {
    let params = Parameters::get();

    let sett = tartan::parse(&params.pattern);

    let mut image: RgbImage = ImageBuffer::new(params.width, params.height);

    for (x, y, colour) in sett.coord_colours(params.width, params.height, params.skip) {
        image.put_pixel(x, y, Rgb(colour.to_array()));
    }

    if !params.ugly {
        image = gaussian_noise(&image, 1.5, 4.5, 1);
        image = imageops::blur(&image, 0.7);
    }
    image.save(params.filename).unwrap();
}
