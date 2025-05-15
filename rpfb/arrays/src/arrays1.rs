fn main() {
    // Example of letting rust infer the type of the elements
    // though they all must be of the same type
    let num_array = [4, 7, -3, 0, 9];

    // Examples of declaring explict type for the array elements
    let fl_array: [f64;5] = [-7.4, 0.3, -5.12, 4.9, 8.3];
    let name_array: [&str;7] = ["Alice", "Bob", "Poonam",  "Dexter", "Willo", "Ian", "Gera"];

    // Examples of filling an array of values
    let num_def_array: [isize;3] = [5;3];
    let fl_def_array: [f64;4] = [0.0;4];
    let string_def_array: [&str;7] = ["NULL";7];
    let boolean_def_array: [bool;4] = [true;4];

    // Print everything
    println!("\nnum_array: {:?}", num_array);
    println!("\nnum_array Size: {}", num_array.len());

    println!("\nfl_array: {:?}", fl_array);
    println!("\nfl_array Size: {}", fl_array.len());

    println!("\nname_array: {:?}", name_array);
    println!("\nname_array Size: {}", name_array.len());

    println!("\nnum_def_array: {:?}", num_def_array);
    println!("\nnum_def_array Size: {}", num_def_array.len());
    
    println!("\nfl_def_array: {:?}", fl_def_array);
    println!("\nfl_def_array Size: {}", fl_def_array.len());
    
    println!("\nstring_def_array: {:?}", string_def_array);
    println!("\nstring_def_array Size: {}", string_def_array.len());
    
    println!("\nboolean_def_array: {:?}", boolean_def_array);
    println!("\nboolean_def_array Size: {}", boolean_def_array.len());
    
}
