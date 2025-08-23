#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let p = Point {x: 5, y: 10 };
	println!("{:?}", p); // now this will work, note the format specifier {:?} needed for debug trait display to work
}

