use crate::garden::vegetables::Asparagus;

// Pull the garden module >> src/garden.rs into scope
pub mod garden;
/*

Note when we declare a module, rust will look for it in the following places:

1. Inline within curly brackets that replace the semicoloon following mod garden
2. In the file src/garden.rs
3/ In the file src/garden/mod.rs 

*/

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
