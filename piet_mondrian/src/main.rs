use mondrian::{save, Mondrian};

fn main() {
    let mut s = Mondrian::default();
    s.line_width = 0.8;
    save("iter-5.svg", &s.generate(4)).unwrap();
}
