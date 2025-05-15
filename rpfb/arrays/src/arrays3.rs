fn main() {
    let mut num_array_f: [f64;5] = [
        1.67,
        -8.45,
        3.76,
        -5.19,
        7.31
    ];

    // Print the original array
    println!("Original array: {:?}", num_array_f);

    // Modify the array
    // Note we can ONLY do this because it is mutable (eg let mut <arrayname>)
    num_array_f[0] = 0.0;
    num_array_f[1] = 6.09;
    num_array_f[2] = 1.73;
    num_array_f[3] = 8.23;
    num_array_f[4] = 2.22;

    // Print array after modification
    println!("Array after modification: {:?}", num_array_f);
}
