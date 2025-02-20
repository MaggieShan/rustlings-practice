// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// Lesson: can reuse existing variable names when converting types, we say the first variable is shadowed by the second (aka. shadowing)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: i32;
    number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
