fn main() {
    let str1 = String::from("Russia is the largest country by area");
    println!("str1: {}", str1);
    let str2 = String::from("Netherlands,Norway,Japan,USA,India,Singapore");
    println!("str2: {}", str2);

    // Lets get some tokens shall we!
    println!("Tokenised str1 by whitespace:\n");
    for str_token in str1.split_whitespace() {
        println!("{}", str_token);
    }
    println!("Tokenised str2 by comma:\n");
    for str_token in str2.split(",") {
        println!("{}", str_token);
    }
}
