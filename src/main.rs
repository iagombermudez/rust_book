struct Point(i32, i32, i32);

fn main() {
    let origin = Point(1, 2, 3);
    let Point(x, y, z) = origin;
    println!("{x} {y} {z}")
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.
