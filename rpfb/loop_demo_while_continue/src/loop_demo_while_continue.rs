fn main() {

    let mut count: isize = 1;
    let mut multiple;

    while count <= 10 {
        multiple = count * 3;
        if multiple % 5 == 0 {
            count += 1;
            continue;
        }
        println!("{}", multiple);
        count += 1;
    }
}
