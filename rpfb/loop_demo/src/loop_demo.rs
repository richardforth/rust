fn main() {
    let mut count: isize = 1;

    loop {
        println!("{}", 5 * count);
        count += 1;
        if count == 11 {
            break;
        }
    }
}
