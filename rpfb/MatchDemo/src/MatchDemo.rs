fn main() {
    let mut num_str = String::new();

    println!("Enter a number:");

    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();

    let num_str = match num {
        0 => "ZERO",
        1 => "ONE",
        2 => "TWO",
        3 => "THREE",
        4 => "FOUR",
        5 => "FIVE",
        6 => "SIX",
        7 => "SEVEN",
        8 => "EIGHT",
        9 => "NINE",
        _ => "An Invalid Input!",
    };
    println!("You have entered: {}\n", num_str);
}
