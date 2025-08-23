enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

// Implementing the clone trait for Animal
impl Clone for Animal {
    fn clone(&self) -> Self {
        match self {
            Animal::Dog(name) => Animal::Dog(name.clone()),
            Animal::Cat(name) => Animal::Cat(name.clone()),
            Animal::Bird(name) => Animal::Bird(name.clone()),
        }
    }
}

fn kind(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog(_) => "Dog",
        Animal::Cat(_) => "Cat",
        Animal::Bird(_) => "Bird",
    }
}

fn sound(animal: Animal) -> &'static str {
    match animal {
        Animal::Dog(_) => "Bark",
        Animal::Cat(_) => "Meow",
        Animal::Bird(_) => "Tweet",
    }
}

fn name(animal: Animal) -> String {
    match animal {
        Animal::Dog(name) => name,
        Animal::Cat(name) => name,
        Animal::Bird(name) => name,
    }
}

fn main() {
    let pet1 = Animal::Dog(String::from("Rover"));
    let pet2 = Animal::Cat(String::from("Tiddles"));
    let pet3 = Animal::Bird(String::from("Charley"));

    // Print everything
    println!("Pet1 is a {}, has the name {}, and makes the sound \"{}\".", 
        kind(pet1.clone()),
        name(pet1.clone()),
        sound(pet1.clone())
    );
    println!("Pet2 is a {}, has the name {}, and makes the sound \"{}\".", 
        kind(pet2.clone()),
        name(pet2.clone()),
        sound(pet2.clone())
    );
    println!("Pet3 is a {}, has the name {}, and makes the sound \"{}\".", 
        kind(pet3.clone()),
        name(pet3.clone()),
        sound(pet3.clone())
    );
}
