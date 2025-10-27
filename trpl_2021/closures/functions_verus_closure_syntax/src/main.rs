fn main() {
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;


    // Check everything
    println!("add_one_v1 (func) with 5 as argument: {}", add_one_v1(5));
    println!("add_one_v2 (closure) with 5 as argument: {}", add_one_v2(5));
    println!("add_one_v3 (closure) with 5 as argument: {}", add_one_v3(5));
    println!("add_one_v4 (closure) with 5 as argument: {}", add_one_v4(5));
}

