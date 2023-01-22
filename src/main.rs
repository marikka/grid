use clap::Parser;
use image::{io::Reader as ImageReader, Rgb};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the image to draw on
    input_image: String,

    /// Name of the image to output
    output_image: String,

    /// Divisions on shorter side
    #[arg(short, long, default_value_t = 4)]
    divisions: u32,

    /// Grid line width
    #[arg(short, long, default_value_t = 8)]
    line_width: u32,
}

fn main() {
    let args = Args::parse();
    let mut image = ImageReader::open(args.input_image)
        .expect("can't open file")
        .decode()
        .expect("can't decode file")
        .into_rgb8();
    let (width, height) = image.dimensions();
    let line_color = Rgb([255u8, 255u8, 255u8]);

    let grid_size = width.min(height) as f32 / args.divisions as f32;

    let divisions = (width as f32 / args.divisions as f32).floor() as i32;
    for x_i in 1..divisions {
        let x = (x_i as f32 * grid_size).floor() as i32;
        let r = Rect::at(x, 0).of_size(args.line_width, height);
        draw_filled_rect_mut(&mut image, r, line_color);
    }

    let divisions = (height as f32 / args.divisions as f32).floor() as i32;
    for y_i in 1..divisions {
        let y = (y_i as f32 * grid_size).floor() as i32;
        let r = Rect::at(0, y).of_size(width, args.line_width);
        draw_filled_rect_mut(&mut image, r, line_color);
    }

    image.save(args.output_image).expect("can't save");
}
