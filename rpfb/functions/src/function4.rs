fn add_two_numbers(a: isize, b: isize) -> isize {
    let sum = a + b;
    return sum;
}

fn main() {
    let x: isize = 34;
    let y: isize = 86;

    let s = add_two_numbers(x, y);
    println!("\nx = {} y = {} sum = {}\n", x, y, s);
}
