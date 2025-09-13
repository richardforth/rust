pub fn greet_name(name: &str) -> String {
     //format!("Hello {name}!")
     String::from("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let greeting = greet_name("Richard");
        assert!(greeting.contains("Richard"));
    }
}
