fn main() {
    let x = 5;
    let result = if x > 0 { "positive" } else { "non-positive"};
    println!("{}", result);

    let mut count = 0;
    loop {
        println!("{}", count);
        count += 1;
        if count == 3 { break; }
    }

    let mut n = 0;
    while n < 3 {
        println!("n = {}", n);
        n += 1;
    }

    for i in 0..3 {
       println!("i = {}", i);
    }
}
