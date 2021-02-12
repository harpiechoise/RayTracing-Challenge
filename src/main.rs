mod math_objects;
use math_objects::tuple::point;

fn main() {
    let a = point(10.0, -4.3, 5.0);
    let b = point(10.0, -4.3, 4.0);
    println!("{}", a == b);
}
