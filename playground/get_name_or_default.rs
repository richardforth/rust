use std::env;

fn main() {
    let name = env::args()
        .nth(1)
        .unwrap_or_else(|| "Richard".to_string());
	
	  println!("Hello there, {}", name); 
}
