fn main() {
    // Example of functions calling each other
    println!("Inside main function, calling function_1.");
    function_1();

    println!("Back inside main function, calling function_4.");
    function_4();
}

fn function_1() {
    println!("Inside function_1, calling function_2.");
    function_2();
}

fn function_2() {
    println!("Inside function_2, calling function_3.");
    function_3();
}

fn function_3() {
    println!("Inside function_3, returning to the calling function.");
}

fn function_4() {
    println!("Inside function_4, calling function_2.");
    function_2();
}
