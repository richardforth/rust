fn main() {
    let city_array: [&str;5] = [
	"Taipei",
	"Casablanca",
	"Oslo",
	"Santiago",
	"Waterloo"
    ];

    println!("\nArry printed using for loop:\n");
    for city in city_array.iter() {
        println!("\n{}", city);
    }
    println!("\n");
}
