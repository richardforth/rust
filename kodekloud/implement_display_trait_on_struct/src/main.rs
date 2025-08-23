use std::fmt;

struct Point {
	x: i32,
	y: i32,
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

fn main() {
	let p = Point {x: 5, y: 10 };
	println!("{}", p); // now this will work with the custom format using the display trait impl
}
