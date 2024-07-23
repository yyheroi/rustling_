fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = "hello";

    if a.len() >= 1 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
