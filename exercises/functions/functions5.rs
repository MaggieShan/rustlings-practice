// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// Lesson: without a semi colon, the expression becomes the return value of the function
// * either add a return statement (usually for early returns)
// * or remove the semi-colon and let the expression be the return value

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
