use nannou::prelude::*;

// const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 500.0;
const N_CIRCLES: usize = 50;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let window = app.window_rect();
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    let points = (0..=N_CIRCLES).map(|_| {
        let x = random_range(window.left(), window.right());
        let y = random_range(window.top(), window.bottom());

        (pt2(x, y), WHITE)
    });

    for (point, color) in points {
        draw.ellipse().color(color).x_y(point.x, point.y);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
