#[derive(Debug)]
enum Direction {
	North,
	South,
	East,
	West,
}

fn main() {
	let dir1 = Direction::North;
	let dir2 = Direction::South;

	println!("{:?}", dir1);
	println!("{:?}", dir2);
}
