fn main() {
    let mut number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    number = "32";
    println!("Number plus two is: {}", number.to_owned() + "2");
}
