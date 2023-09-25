// gfx libs
use macroquad::{prelude::*, miniquad::window::set_window_size};
// used to read the file
use std::fs::File;
use std::io::*;
// used for bit opperations
#[path ="../../conversion.rs"]
mod conversion;
use crate::conversion::*;
// Error message constants
const INVALID_DIMENSIONS: &str = "ERROR: invalid image dimensions";
const NOPIXEL: &str = "ERROR: expected pixel value";

#[macroquad::main("imageload")]
async fn main() {
    let file = File::open("image.mrgb").unwrap();
    let contents = file.bytes().into_iter().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let size = contents.len();
    let mut contents = contents.into_iter();
    let width = concat_u8(0, contents.next().expect(INVALID_DIMENSIONS), contents.next().expect(INVALID_DIMENSIONS));
    // Gets the height given the width and file size.
    // We subtract two because the first two bytes of the file are used to determine the width of
    // the file; then,  we divide by three and multiply by two because three bytes are used to represent
    // the color of two pixels. The result will be the number of pixels in the image and we can
    // divide that by the width to find the height.
    let height = (((size as u32 - 2) / 3) * 2) / width;

    let pixels = (0..((size - 2) / 3)).into_iter().map(|_| {
        let mut pixel_data = concat_u8(
            contents.next().unwrap(),
            contents.next().unwrap(),
            contents.next().unwrap()
            );
        // we decode 6 colors every 3 bytes because 3 bytes is 24 bits and each color channel is 4
        // bits we can get 6 color channels with 3 bytes 4 * 6 = 3 * 8
        (0..6).into_iter().map( |index| {
            let b = pixel_data >> ((19 + (index > 3) as u32) - (index * 4) );
            pixel_data -= b << ((19 + (index > 3) as u32) - (index * 4));
            return b as u8;
        }).collect::<Vec<u8>>()
    }).flatten().collect::<Vec<u8>>();

    set_window_size(width, height);
    loop {
        for y in 0..height as usize {
            for x in 0..width as usize {
                let pos = ((y * width as usize) + x) * 3;
                draw_rectangle(x as f32, y as f32, 1., 1., Color { r: pixels[pos] as f32/ 15., g: pixels[pos + 1] as f32 / 15., b: pixels[pos + 2] as f32 / 15., a: 1.0})
            }
        }
        next_frame().await;
    }
}
