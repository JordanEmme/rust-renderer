use super::maths_utils;
use super::mesh::{Coords2D, Coords3D, Mesh, Triangle};
use super::tga;

pub fn line(x0: u16, y0: u16, x1: u16, y1: u16, img: &mut tga::Image<tga::Rgb>, color: tga::Rgb) {
    let mut start_x: u16 = x0;
    let mut start_y: u16 = y0;
    let mut end_x: u16 = x1;
    let mut end_y: u16 = y1;

    let mut transposed: bool = false;
    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        (start_x, start_y) = (start_y, start_x);
        (end_x, end_y) = (end_y, end_x);
        transposed = true;
    }
    if start_x > end_x {
        (start_x, end_x) = (end_x, start_x);
        (start_y, end_y) = (end_y, start_y);
    }
    let dx: i32 = (end_x - start_x).into();
    let derror2: i32 = (end_y.abs_diff(start_y) * 2).into();
    let mut error2: i32 = 0i32;
    let mut y: u16 = start_y;
    for x in start_x..end_x {
        if transposed {
            let _ = img.set(y, x, color);
        } else {
            let _ = img.set(x, y, color);
        }
        error2 += derror2;
        if error2 > dx {
            if end_y > start_y {
                y += 1;
            } else {
                y -= 1;
            }
            error2 -= dx * 2;
        }
    }
}
