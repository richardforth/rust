fn main() {
    let mut i: isize = 0;
    let mut j: isize;
    let mut k: isize;
    print!("\n");
    loop {
        for _a in i..=5 {
            print!(" ");
        }
        j = 0;
        while j < i {
            print!("{}", j);
            j += 1;
        }
        k = j;
        while k >= 0 {
            print!("{}", k);
            k -= 1;
        }
        i += 1;
        if i == 6 {
            break;
        }
        print!("\n");
    }
    print!("\n");
}
