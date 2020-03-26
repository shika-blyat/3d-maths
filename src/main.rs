mod png_gen;
use png_gen::gen_png;

fn draw_pixel(x: usize, y: usize, vec: &mut [u8], width: usize, (r, g, b): (u8, u8, u8)) {
    vec[(500 - x) * 3 * width + (500 - y) * 3] = r;
    vec[(500 - x) * 3 * width + (500 - y) * 3 + 1] = g;
    vec[(500 - x) * 3 * width + (500 - y) * 3 + 2] = b;
}
fn draw_line(
    (begin_x, begin_y): (f32, f32),
    (end_x, end_y): (f32, f32),
    vec: &mut [u8],
    width: usize,
) {
    // y = ax + b or y = gradient * x + offset
    let gradient = (end_y - begin_y) / (end_x - begin_x);
    let (biggest_x, smallest_x) = if begin_x > end_x {
        (begin_x, end_x)
    } else {
        (end_x, begin_x)
    };
    let (_, mut smallest_y) = if begin_y > end_y {
        (begin_y, end_y)
    } else {
        (end_y, begin_y)
    };
    for x in (smallest_x as u32..biggest_x as u32).rev() {
        smallest_y += gradient;
        draw_pixel(
            x as usize,
            smallest_y.round() as usize,
            vec,
            width,
            (255, 255, 255),
        );
    }
}

fn main() {
    const HEIGHT: usize = 500;
    const WIDTH: usize = 500;
    let mut vec = vec![0; (HEIGHT * WIDTH) * 3];
    draw_line((400., 250.), (250., 400.), &mut vec, WIDTH);
    gen_png(&vec, WIDTH as u32, HEIGHT as u32);
}
