use super::tga;
use std::iter;

pub fn draw_line(x0: u16, y0: u16, x1: u16, y1: u16, img: &mut tga::Image<tga::Rgb>, color: tga::Rgb) {
    iter::successors(Some(0f64), |i| {
        let next: f64 = i + 0.1f64;
        (next < 1f64).then_some(next)
    })
    .for_each(|step| {
        let pixel_x: u16 = x0 + ((x1 - x0) as f64 * step) as u16;
        let pixel_y: u16 = y0 + ((y1 - y0) as f64 * step) as u16;
        let _ = img.set(pixel_x, pixel_y, color);
    })    
}


