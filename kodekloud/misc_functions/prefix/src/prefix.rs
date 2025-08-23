fn main() {
    prefix("PASS", "The process succeeded.");
    prefix("FAIL", "The process failed.");
}

fn prefix(p: &str, m: &str) {
    println!("[{}] {}", p, m);
}
