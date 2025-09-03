use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let floaty_mcfloat_face = 3.14;
    println!("The default float type is {}", 
        type_of(&floaty_mcfloat_face)); // The default float type is f64
}
