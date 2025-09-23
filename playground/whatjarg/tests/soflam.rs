use whatjarg;

#[test]
fn soflam_expands_correctly_caps() {
     let test_string = String::from("SOFLAM");
     let expected_text = String::from("Aquisition Marker");
     let expansion = whatjarg::get_jarg(&test_string);
        assert!(
            expansion.contains(&expected_text),
            "expansion not contain expected text: `{expected_text}`  value was `{expansion}`"
        );
}

#[test]
fn soflam_expands_correctly_lower() {
     let test_string = String::from("soflam");
     let expected_text = String::from("Aquisition Marker");
     let expansion = whatjarg::get_jarg(&test_string);
        assert!(
            expansion.contains(&expected_text),
            "expansion not contain expected text: `{expected_text}`  value was `{expansion}`"
        );
}
