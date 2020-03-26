mod png_gen;

use png_gen::gen_png;
fn is_in_line(
    (begin_x, begin_y): (f32, f32),
    (end_x, end_y): (f32, f32),
) -> impl Fn(f32, f32) -> bool {
    // y = ax + b or y = gradient * x + offset
    let gradient = (end_y - begin_y) / (end_x - begin_x);
    let offset = end_y - (gradient * end_x);
    let (biggest_x, smallest_x) = if begin_x > end_x {
        (begin_x, end_x)
    } else {
        (end_x, begin_x)
    };
    let (biggest_y, smallest_y) = if begin_y > end_y {
        (begin_y, end_y)
    } else {
        (end_y, begin_y)
    };
    move |x, y| {
        if x > biggest_x || x < smallest_x || y < smallest_y || y > biggest_y {
            false
        } else {
            y == x * gradient + offset
        }
    }
}

fn main() {
    let mut vec = vec![];
    let begin_coord = (400f32, 250f32);
    let end_coord = (250f32, 400f32);
    let is_in = is_in_line(begin_coord, end_coord);
    let height = 500;
    let width = 500;
    let mut cntr = 0;
    for y in (0..height).rev() {
        for x in 0..width {
            if is_in(x as f32, y as f32) {
                cntr += 1;
                vec.push(150);
                vec.push(200);
                vec.push(100);
            } else {
                vec.push(0);
                vec.push(0);
                vec.push(0);
            }
        }
    }
    println!("{}", cntr);
    gen_png(vec, width);
}
