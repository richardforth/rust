fn main() {
    let num_array = [5, 9, 1, 0, 4, 2, 1, 8];
    let fl_array: [f64; 4] = [11.2, 87.4, 91.6, 32.5];
    let country_array: [&str; 5] = ["India", "Greece", "Thailand", "Armenia", "Ukraine"];

    // Slice the arrays
    let num_array_slice = &num_array[2..6];
    let fl_array_slice = &fl_array[1..3];
    let country_array_slice = &country_array[0..4];

    // Print everything
    println!(
        "\nnum_array: {:?}\nnum_array Size: {}",
        num_array,
        num_array.len()
    );
    println!(
        "num_array_slice: {:?}\nnum_array_slice Size: {}",
        num_array_slice,
        num_array_slice.len()
    );

    println!(
        "\nfl_array: {:?}\nfl_array Size: {}",
        fl_array,
        fl_array.len()
    );
    println!(
        "fl_array: {:?}\nfl_array_slice Size: {}",
        fl_array_slice,
        fl_array_slice.len()
    );

    println!(
        "\ncountry_array: {:?}\ncountry_array Size: {}",
        country_array,
        country_array.len()
    );
    println!(
        "country_array_slice: {:?}\ncountry_array_slice Size: {}",
        country_array_slice,
        country_array_slice.len()
    );
}
