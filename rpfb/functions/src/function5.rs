fn array_sum(num_array: [isize; 5]) -> isize {
    let mut sum = 0;
    for x in num_array.iter() {
        sum = sum + x;
    }
    return sum;
}

fn main() {
    let arr = [1, 8, 5, 3, 7];
    let s = array_sum(arr);
    println!("\narr:\n {:?}\n\nSum = {}", arr, s);
}
