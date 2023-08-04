use nannou::prelude::*;

const SIZE: u32 = 320;
const WIDTH: u32 = 320;
const HEIGHT: u32 = 320;
const LINE_WIDTH: f32 = 2.0;
const GAP: u32 = SIZE / 8;

fn main() {
    nannou::sketch(view)
        .size(WIDTH, HEIGHT)
        .loop_mode(LoopMode::loop_once())
        .run()
}

#[derive(Clone, Copy)]
struct Dot {
    x: f32,
    y: f32,
}

fn view(app: &App, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw().x_y(-window.w() * 0.5, -window.h() * 0.5);

    draw.background().color(SNOW);

    let mut odd = false;
    let mut lines = Vec::<Vec<Dot>>::new();

    for y in (GAP / 2..=SIZE).step_by(GAP as usize) {
        odd = !odd;
        let mut dots = Vec::<Dot>::new();
        for x in (GAP / 4..=SIZE).step_by(GAP as usize) {
            let dot = Dot {
                x: x as f32 + (if odd { GAP as f32 / 2.0 } else { 0.0 }),
                y: y as f32,
            };
            dots.push(dot);
        }
        lines.push(dots);
    }

    for y in 0..lines.len() - 1 {
        odd = !odd;
        let mut dot_lines = Vec::<Dot>::new();
        for i in 0..lines[y].len() {
            dot_lines.push(if odd { lines[y][i] } else { lines[y + 1][i] });
            dot_lines.push(if odd { lines[y + 1][i] } else { lines[y][i] });
        }
        for i in 0..dot_lines.len() - 2 {
            draw_triangle(&draw, dot_lines[i], dot_lines[i + 1], dot_lines[i + 2])
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_triangle(draw: &Draw, point_a: Dot, point_b: Dot, point_c: Dot) {
    let a = pt2(point_a.x, point_a.y);
    let b = pt2(point_b.x, point_b.y);
    let c = pt2(point_c.x, point_c.y);
    // let d = pt2(point_a.x, point_a.y);

    draw.tri()
        .no_fill()
        .points(a, b, c)
        .stroke_weight(LINE_WIDTH)
        .color(BLACK);
}
