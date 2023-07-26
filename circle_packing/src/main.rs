use nannou::color::{POWDERBLUE, WHITESMOKE};
use nannou::prelude::*;

const LINE_WIDTH: f32 = 2.0;
const MIN_RADIUS: f32 = 2.0;
const MAX_RADIUS: f32 = 385.0;
const N_CIRCLES: usize = 3000;
const CREATE_CIRCLE_ATTEMPTS: usize = 500;

struct Model;

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
}

impl Circle {
    fn collides(&self, other: &Circle) -> bool {
        let a = self.radius + other.radius;
        let x = self.x - other.x;
        let y = self.y - other.y;

        if a >= ((x * x) + (y * y)).sqrt() {
            true
        } else {
            false
        }
    }

    fn any_collision(&self, others: &Vec<Circle>) -> bool {
        for other in others {
            if self.collides(other) {
                return true;
            }
        }
        false
    }
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().view(view).build().unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(WHITE);

    let mut circles: Vec<Circle> = Vec::with_capacity(N_CIRCLES);

    for _ in 0..=N_CIRCLES {
        for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
            let x = random_range(window.left(), window.right());
            let y = random_range(window.top(), window.bottom());
            let radius = random_range(MIN_RADIUS, MAX_RADIUS);

            let c = Circle {
                x: x,
                y: y,
                radius: radius,
            };

            if c.any_collision(&circles) {
                continue;
            }

            circles.push(c);
            break;
        }
    }

    for circle in circles {
        let line_points = (0..=360).map(|i| {
            // Convert each degree to radians.
            let radian = deg_to_rad(i as f32);
            // Get the sine of the radian to find the x co-ordinate of this point of the circle
            // and multiply it by the radius.
            let x_ = circle.x + radian.sin() * circle.radius;
            // Do the same with cosine to find the y co-ordinate.
            let y_ = circle.y + radian.cos() * circle.radius;
            // Construct and return a point object with a color.
            (pt2(x_, y_), BLACK)
        });

        draw.polyline()
            .weight(LINE_WIDTH)
            .points_colored(line_points);
    }
    draw.to_frame(app, &frame).unwrap();
}
