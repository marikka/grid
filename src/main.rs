use image::{io::Reader as ImageReader, GenericImageView, Rgb};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).unwrap();
    println!("Opening file {}", file_name);
    let mut img = ImageReader::open(file_name)
        .expect("can't open file")
        .decode()
        .expect("can't decode file");

    println!("{:?}", img.color());
    let (width, height) = img.dimensions();
    let i2 = img.as_mut_rgb8().expect("can't convert to rgba8");
    let red = Rgb([255u8, 255u8, 255u8]);

    let divisor = 4;
    let line_width = 8;
    let grid_size = width.min(height) as f32 / divisor as f32;

    let divisions = (width as f32 / divisor as f32).floor() as i32;
    for x_i in 1..divisions {
        let x = (x_i as f32 * grid_size).floor() as i32;
        let r = Rect::at(x, 0).of_size(line_width, i2.height());
        draw_filled_rect_mut(i2, r, red);
    }

    let divisions = (height as f32 / divisor as f32).floor() as i32;
    for y_i in 1..divisions {
        let y = (y_i as f32 * grid_size).floor() as i32;
        let r = Rect::at(0, y).of_size(i2.width(), line_width);
        draw_filled_rect_mut(i2, r, red);
    }

    i2.save("image2.jpg").expect("can't save");
}
