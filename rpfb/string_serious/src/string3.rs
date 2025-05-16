fn main() {
    let str1 = String::from("What they said is, together they can win!");
    println!("str1: {}", str1);
    let str2 = str1.replace("they", "we");
    println!("str2: {}", str2);
}
