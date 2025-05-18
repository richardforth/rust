struct Country {
    name: String,
    capital: String,
    continent: String,
    calling_code: i32,
}

// A helper function to display a struct
// A bit like a "getter"
fn show_country(c: Country) {
    println!("Country Name: {}", c.name);
    println!("Capital: {}", c.capital);
    println!("Continent: {}", c.continent);
    println!("Calling Code: +{}\n", c.calling_code);
}

// setters in main
fn main() {
    let c1 = Country {
        name: String::from("Australia"),
        capital: String::from("Canberra"),
        continent: String::from("Australia"),
        calling_code: 61,
    };

    let c2 = Country {
        name: String::from("UK"),
        capital: String::from("London"),
        continent: String::from("Europe"),
        calling_code: 44,
    };

    let c3 = Country {
        name: String::from("South Africa"),
        capital: String::from("Cape Town"),
        continent: String::from("Africa"),
        calling_code: 27,
    };

    let c4 = Country {
        name: String::from("Japan"),
        capital: String::from("Tokyo"),
        continent: String::from("Asia"),
        calling_code: 81,
    };

    let c5 = Country {
        name: String::from("Canada"),
        capital: String::from("Ottawa"),
        continent: String::from("North America"),
        calling_code: 1,
    };

    show_country(c1);
    show_country(c2);
    show_country(c3);
    show_country(c4);
    show_country(c5);
}
