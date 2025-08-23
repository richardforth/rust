struct Rectangle 
{
	width: u32,
	height: u32,
}

fn main() {
	let r1 = Rectangle {
		width: 30,
		height: 50,
	};

	let r2 = Rectangle {
		height: 60,
		..r1
	};

	println!("r2 Width: {}", r2.width);
	println!("r2 Height: {}", r2.height);
}
