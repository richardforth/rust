fn main() {
    let float_vector = vec![3.59, -6.25, -0.23, 5.69, 8.42, 1.23];
    let int_vector = vec![0, 10, 20];

    let mut i = 0;

    // while loop
    while i < float_vector.len() {
        println!("{}", float_vector[i]);
        i = i + 1;
    }

    // for loop
    for num in int_vector {
        println!("{}", num);
    }
}
