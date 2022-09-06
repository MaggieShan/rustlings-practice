// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

// Lesson: 
// * Rust code uses snake case
// * functions can be defined before or after where they're used, 
//   as long as they're defined somewhere in the scope that can be seen by the caller

fn call_me2() {
    println!("Hello to you 2!");
}

fn main() {
    call_me();
    call_me2();
}

fn call_me() {
    println!("Hello, World!");
}
