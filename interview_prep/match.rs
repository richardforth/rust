fn main() {
    let num = 5;
    match num {
        1 => println!("one"),
        2 | 3 => println!("two or three"),
        4..=10 => println!("between four and ten"),
        _ => println!("something else"),
    }
}
