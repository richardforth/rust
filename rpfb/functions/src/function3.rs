fn main() {
    let a: isize = 21;
    println!("Inside main function.\na = {}", a);
    show_triple(a);
}

fn show_triple(a: isize) {
    println!(
        "\nInside show_triple function.\na = {}, (a x 3) = {}\n",
        a,
        (a * 3)
    );
}
