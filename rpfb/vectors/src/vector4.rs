fn main() {
    // Vectors must be mutable in order for push to work
    let mut names: Vec<&str> = Vec::new();
    let mut num_vector = vec![1, 0, 3];

    println!(
        "\nnames length: {}\nnum_vector length: {}",
        names.len(),
        num_vector.len()
    );

    // Push some names onto the names vector
    names.push("Igor");
    names.push("Bilal");
    names.push("Silvy");
    names.push("Bob");

    println!("\nnames: {:?}", names);
    println!("\num_vector: {:?}", num_vector);

    println!(
        "\nnames length: {}\nnum_vector length: {}",
        names.len(),
        num_vector.len()
    );

    // Push some numbers onto num_vector
    num_vector.push(2);
    num_vector.push(5);

    println!("\nnames: {:?}", names);
    println!("\nnum_vector: {:?}", num_vector);
    println!(
        "\nnames length: {}\nnum_vector length: {}",
        names.len(),
        num_vector.len()
    );
}
