fn main() {
    let dow = 5; // dow = day of week
    match dow {
        0 | 7 => println!("Sunday"),
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        _ => eprintln!("Error: not a valid day"),
    }
}
