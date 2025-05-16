fn main() {
    println!("\nConcatenation using + operator:\n");
    let str1 = "This is a".to_string();
    println!("\nstr1: {}", str1);
    let str2 = "string".to_string();
    println!("\nstr2: {}", str2);
    let str3 = "demo".to_string();
    println!("\nstr3: {}", str3);

    // Here lieth magic and mystery....
    let str4 = str1 + " " + &str2 + " " + &str3;
    println!("Concatenated string: {}\n", str4);
    println!("Concatenation using push_str and push functions:\n");

    let str5: &str = "Another";
    println!("\nstr5: {}", str5);
    let str6: &str = "concatenation";
    println!("\nstr6: {}", str6);
    let str7: &str = "demo";
    println!("\nstr7: {}", str7);
    let mut str8 = String::new();
    str8.push_str(str5);
    str8.push(' ');
    str8.push_str(str6);
    str8.push(' ');
    str8.push_str(str7);
    str8.push(' ');
    println!("Concatenated string: {}\n", str8);
}
