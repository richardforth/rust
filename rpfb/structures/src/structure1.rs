struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };

    let p2 = Point { x: 6, y: 8 };
    println!("\nPoint p1: \nx = {}\ty = {}", p1.x, p1.y);
    println!("\nPoint p2: \nx = {}\ty = {}", p2.x, p2.y);
}
