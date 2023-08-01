use svg::save;

fn main() {
    let mut s = piet_mondrian::Mondrian::default();
    save("piet_mondrian.svg", &s.generate(5)).unwrap();
}
