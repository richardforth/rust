fn main() {
    // Several different methods of getting or converting to a String object
    let name1: &str = "Orin";
    let mut name2 = String::new();
    name2.push_str("Deb");
    let name3 = String::from("Lola");
    let name4 = "Jade".to_string();
    let num: f64 = 54.75;
    let num_str = num.to_string();

    // Print all strings and their respective sizes
    println!("name1: {}\t Length: {}", name1, name1.len());
    println!("name2: {}\t Length: {}", name2, name2.len());
    println!("name3: {}\t Length: {}", name3, name3.len());
    println!("name4: {}\t Length: {}", name4, name4.len());
    println!("num_str: {}\t Length: {}", num_str, num_str.len());
}
