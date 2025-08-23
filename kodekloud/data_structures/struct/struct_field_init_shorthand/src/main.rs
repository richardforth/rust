struct Rectangle 
{
	width: u32,
	height: u32,
}

fn main() {
	let width = 30;
	let height = 50;
	let r1 = Rectangle {
		width,
		height
	};

	println!("Width: {}", r1.width);
	println!("Height: {}", r1.height);
}
