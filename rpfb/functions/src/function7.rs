fn main() {
    // Example of functions calling each other
    // However this isnt the right code in the book
    // It appears that this code and the next code are duplicated

    // This code was supposed to be about returning vectors from functions.
    // I will persevere, and copy this code for the next exercise. But I
    // will need to come back to this to run a test to return a Vector type.
    // even if I have to ask ChatGPT, as the correct example is missing in the book.
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
