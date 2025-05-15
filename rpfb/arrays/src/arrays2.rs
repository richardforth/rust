fn main() {
    // thought I'd try and see if pretty-printing
    // the array actually compiles, be nice if it does 
    let num_array: [i32;10] = [
	15,
	-76,
	-23,
	26,
	82,
	-54,
	34,
	0,
	64,
	96
    ];

    println!("\nElement at index 0: {}", num_array[0]);
    println!("\nElement at index 1: {}", num_array[1]);
    println!("\nElement at index 2: {}", num_array[2]);
    println!("\nElement at index 3: {}", num_array[3]);
    println!("\nElement at index 4: {}", num_array[4]);
    println!("\nElement at index 5: {}", num_array[5]);
    println!("\nElement at index 6: {}", num_array[6]);
    println!("\nElement at index 7: {}", num_array[7]);
    println!("\nElement at index 8: {}", num_array[8]);
    println!("\nElement at index 9: {}", num_array[9]);
}
