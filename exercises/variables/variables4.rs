// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

// Lesson: variables are immutable by default but can be changed with keyword mut

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
