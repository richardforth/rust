fn main() {
    let num_array = [10, 45, 0, 24, 75, 12];
    let fl_array = [-5.7, -9.6, 5.8, 1.3, 4.5];

    let num_slice = &num_array[0..3];
    let fl_slice = &fl_array[1..4];

    println!(
        "\nnum_array: {:?}\nnum_array Size: {}\n",
        num_array,
        num_array.len()
    );

    println!("\nnum_slice (ref &num_array[0..3]) printed using while loop: \n");
    let mut i = 0;
    while i < num_slice.len() {
        println!("Element at slice index {}: {}", i, num_slice[i]);
        i = i + 1;
    }
    println!(
        "\nfl_array: {:?}\nfl_array Size: {}\n",
        fl_array,
        fl_array.len()
    );
    println!("\nfl_slice (ref &fl_array[1..4]) printed using for loop: \n");
    for element in fl_slice {
        println!("{}", element);
    }
    println!();
}
