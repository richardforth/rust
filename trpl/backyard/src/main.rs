use crate::garden::vegetables::Asparagus;

// Pull the garden module >> src/garden.rs into scope
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);
}
