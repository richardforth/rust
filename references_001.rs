fn main() {
    let mut s = String::from("Rust");
    let r1 = &s;
    let r2 = &mut s;
    println!("{}", r1);
    r2.push_str(" language"); 
}

/*

Firt commit: Original Book Code asking to fix code that will not compile

*/
