pub fn add_two(a: i32) -> i32 {
    a + 2
}

// Note that addtwo below is not marked as pub
fn addtwo(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_add() {
        let result = addtwo(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn public_add() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
