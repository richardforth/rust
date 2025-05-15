fn main() {
    let mut num_str = String::new();
    let mut i = 0;
    let mut sum: isize = 0;
    let avg: f64;
    let mut num_array: [isize; 5] = [0; 5];

    while i < 5 {
        println!("\nEnter a number at index {}:", i);
        // Clear num_str so that previous input is flushed.
        num_str.clear();
        std::io::stdin().read_line(&mut num_str).unwrap();

        num_array[i] = num_str.trim().parse().unwrap();
        sum = sum + num_array[i];
        i += 1;
    }
    avg = (sum as f64) / 5.0;

    // Print everything
    println!("\nnum_array: {:?}", num_array);
    println!("\nnum_array Size: {}", num_array.len());
    println!("\nSum of elements: {}", sum);
    println!("\nAverage: {}", avg);
}
