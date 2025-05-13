fn main() {

    let mut count: isize = 1;
    let mut multiple;

    println!("{} {}", "count", "multiple");
    println!("-----------------------");
    while count <= 10 {
        multiple = count * 3;
        if multiple % 5 == 0 {
            println!("   {}    {} ==>> SKIPPED", count, multiple);
            count += 1;
            continue;
        }
        println!("   {}    {}", count, multiple);
        count += 1;
    }
}
