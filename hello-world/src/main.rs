
use std::io;
use rand::Rng;

// We can create a project using cargo new.
// We can build a project using cargo build.
// We can build and run a project in one step using cargo run.
// We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
fn main() {
    println!("Guess the number!");

    // we call the rand::thread_rng function that gives us the particular random number generator we’re going to use: 
    // one that is local to the current thread of execution and is seeded by the operating system
    let _secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    // we’ll create a variable to store the user input
    // We use the let statement to create the variable
    // In Rust, variables are immutable by default, meaning 
    // once we give the variable a value, the value won’t change
    // To make a variable mutable, we add mut before the variable name
    let mut guess = String::new();

    // The & indicates that this argument is a reference, 
    // which gives you a way to let multiple parts of your code access 
    // one piece of data without needing to copy that data into memory 
    // multiple times.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
