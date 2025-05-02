fn main() {
 // Constants and Immutable Variables are different
 // At compile time constants are replaced by their values
 //  whereas immutable variable values are referenced by
 //  their address in memory.
 // Constants by convention should be uppercase, they can
 //  be lowercase but the compiler will show a warning, but
 //  it will compile just fine. 
 const FIRSTNAME:&str = "Richard";
 const MIDDLE_INITIAL:&str = "A";
 const SURNAME: &str = "Forth";
 println!("My name is {} {}. {}", FIRSTNAME, MIDDLE_INITIAL, SURNAME);
}
