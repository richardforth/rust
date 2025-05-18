struct Data {
    name: String,
    age: i32,
    gender: char,
    city: String,
    country: String,
}

// A function that returns a Structure variable we have defined
// A true stetter
// Personally I would have rather called this set_data as its clearly a setter.
fn get_data() -> Data {
    let dl = Data {
        name: String::from("Antonio"),
        age: 33,
        gender: 'M',
        city: String::from("Milan"),
        country: String::from("Italy"),
    };
    return dl;
}

// A getter, I would have called this get_data as its clearly a getter
// as a sister function to the setter set_data
fn show_data(p: Data) {
    println!(
        "\nname: {}\nage: {}\ngender: {}\ncity: {}\ncountry: {}",
        p.name, p.age, p.gender, p.city, p.country
    );
}

fn main() {
    // Actually for this to be a true setter you'd expect the setter function to take args
    let d = get_data();
    show_data(d);
    // But this is a contrived simple example using structs anyway.
}
