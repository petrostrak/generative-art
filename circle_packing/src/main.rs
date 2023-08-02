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

    fn any_collision(&self, others: &Vec<Circle>, window: Rect) -> bool {
        for other in others {
            if self.collides(other) {
                return true;
            }
        }
        if self.x + self.radius >= window.right() || self.x - self.radius <= window.left() {
            return true;
        }
        if self.y + self.radius >= window.top() || self.y - self.radius <= window.bottom() {
            return true;
        }

        false
    }
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app.new_window().size(500, 500).view(view).build().unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();

    draw.background().color(SNOW);

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);

    for _ in 0..=N_CIRCLES {
        for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
            let x = random_range(window.left(), window.right());
            let y = random_range(window.top(), window.bottom());
            let radius = random_range(MIN_RADIUS, MAX_RADIUS);

            let c = Circle { x, y, radius };

            if c.any_collision(&circles, window) {
                continue;
            }

            circles.push(c);
            break;
        }
    }

    for circle in circles {
        let line_points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x_ = circle.x + radian.sin() * circle.radius;
            let y_ = circle.y + radian.cos() * circle.radius;

            (pt2(x_, y_), BLACK)
        });

        draw.polyline()
            .weight(LINE_WIDTH)
            .points_colored(line_points);
    }
    draw.to_frame(app, &frame).unwrap();
}
