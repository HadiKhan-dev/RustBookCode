fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 1, z: 2 };

    match origin {
        Point { y, .. } => println!("y is {}", y),
    }
}
