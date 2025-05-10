fn main() {
    let mut num_str = String::new();

    println!("Enter a number:");

    std::io::stdin().read_line(&mut num_str).unwrap();

    let num: isize = num_str.trim().parse().unwrap();

    if (num > 0) && (num <= 10) {
        println!("\n{} is in the range of ONE to TEN.\n", num);
    } else if (num > 10) && (num < 20) {
        println!("\n{} is in the range of ELEVEN to TWENTY.\n", num);
    } else if (num > 20) && (num <= 30) {
        println!("\n{} is in the range of TWENTY ONE to THIRTY.\n", num);
    } else if (num > 30) && (num <= 40) {
        println!("\n{} is in the range of THIRTY ONE to FORTY.\n", num);
    } else if (num > 40) && (num <= 50) {
        println!("\n{} is in the range of FORTY ONE to FIFTY.\n", num);
    } else if (num > 50) && (num <= 60) {
        println!("\n{} is in the range of FIFTY ONE to SIXTY.\n", num);
    } else if (num > 60) && (num <= 70) {
        println!("\n{} is in the range of SIXTY ONE to SEVENTY.\n", num);
    } else if (num > 70) && (num <= 80) {
        println!("\n{} is in the range of SEVENTY ONE to EIGHTY.\n", num);
    } else if (num > 80) && (num <= 90) {
        println!("\n{} is in the range of EIGHTY ONE to NINETY.\n", num);
    } else if (num > 90) && (num <= 100) {
        println!("\n{} is in the range of NINETY-ONE to ONE HUNDRED.\n", num);
    } else {
        println!(
            "\n{} is either less than ONE or greater than ONE HUNDRED",
            num
        );
    }
}
