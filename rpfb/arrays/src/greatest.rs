fn main() {
    let mut num_str = String::new();
    let mut i = 0;
    let mut greatest: isize = -999;
    let mut num_array: [isize; 5] = [0; 5];

    while i < 5 {
        println!("\nEnter a number at index {}", i);
        num_str.clear(); // flush input previous buffer for memory safety.
        std::io::stdin().read_line(&mut num_str).unwrap();
        num_array[i] = num_str.trim().parse().unwrap();

        if num_array[i] > greatest {
            greatest = num_array[i];
        }
        i += 1
    }
    println!("\nnum_array: {:?}\nGreatest: {}", num_array, greatest);
}
