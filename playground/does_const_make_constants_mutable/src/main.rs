fn main() {
    const mut PI: f64 = 3.1452;
    PI = 3.14;

    println!("No! because traditionally CONSTANTS dont change...");
    println!("...notwithstanding rusts default imutability stance,");
    println!("by definition any constant that changes is not a constant,");
    println!("but a variable...read the last sentence again, slowly.");

    // note this program does not compile, it highlights some terrible advice
    // to compile remove mut and comment the second assignment to PI
    println!("\n#TerribleAdviceFromBooks");
}
