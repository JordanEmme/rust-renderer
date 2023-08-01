pub mod tga;
pub mod line_drawer;

use std::fs::File;
use std::io::BufWriter;

const RED: tga::Rgb = tga::Rgb {r: 255, g: 0, b: 0};

fn main() {
    let mut img = tga::Image::<tga::Rgb>::new(10,10);
    line_drawer::line(0, 0, 9,9, &mut img, RED);

//    let mut img = tga::Image::<tga::Rgb>::new(10, 10);
//    let _ = img.set(0, 0, RED);

    let output_filename = "output.tga";
    let mut writer = BufWriter::new(File::create(output_filename).unwrap());
    img.write(&mut writer, true, false).unwrap();
}
