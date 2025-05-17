fn main() {
    // Returning a vector type
    // Code example missing in book due to a duplicate code snippet
    // So Im grifting this one based on my knowledge so far
    println!("Creating some variables to pass to array...");
    let a: isize = 34;
    let b: isize = 76;
    let c: isize = -24;
    let d: isize = 12;
    let myarr = create_new_vector(a, b, c, d);
    println!("myarr: {:?}", myarr);
    println!("myarr Size: {}", myarr.len());
}
fn create_new_vector(a: isize, b: isize, c: isize, d: isize) -> Vec<isize> {
    let mut num_vector: Vec<isize> = Vec::new();
    num_vector.push(a);
    num_vector.push(b);
    num_vector.push(c);
    num_vector.push(d);
    return num_vector;
}
