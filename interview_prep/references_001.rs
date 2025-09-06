fn main() {
    let mut s = String::from("Rust");
    println!("{}", &s);
    let r2 = &mut s;
    r2.push_str(" language"); 
    println!("{}", r2);
}

/*

First commit: Original Book Code asking to fix code that will not compile
Second Commit: Fix so it compiles: 
                 - Put r1 and its println macro in its own scope,
                 -  add a println macro for r2
Third commit: Alternative fix without a scope, print directly from &s
                before immutable borrow
*/
