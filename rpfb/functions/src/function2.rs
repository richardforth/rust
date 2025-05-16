fn function1() {
    println!("Inside function1.");
}

fn function2() {
    println!("Inside function2.");
}

fn main() {
    println!("Functions can be placed any where in the file\n(above, below, or both above and below, but not inside main function).");
    println!("Inside main function.");
    function1();
    function2();
    function3();
}

fn function3() {
    println!("Inside function3.");
}
