use tartan::colours::Palette;
use tartanlib as tartan;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::console::{time, time_end, time_log};

mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parsetartan(pattern: &str, width: u32, height: u32) {
    time();
    let document = web_sys::window().unwrap().document().unwrap_throw();
    let canvas = document.get_element_by_id("canvas").unwrap_throw();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap_throw();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap_throw();

    time_log();
    let palette = Palette::default();
    let skip = 0;
    let sett = tartan::parse(pattern);
    let mut pixels: Vec<u8> = vec![0; (height * width * 4) as usize];
    time_log();

    for (x, y, colour) in sett.coord_colours(width, height, skip) {
        let [r, g, b] = colour.to_array(&palette);
        let index = (y * (width * 4) + x * 4) as usize;
        pixels[index] = r;
        pixels[index + 1] = g;
        pixels[index + 2] = b;
        pixels[index + 3] = 255;
    }
    time_log();

    let image = web_sys::ImageData::new_with_u8_clamped_array(Clamped(&pixels), width).unwrap();
    context.put_image_data(&image, 0.0, 0.0).unwrap_throw();
    time_end();
}
