
use std::io;
use rand::Rng;
use std::cmp::Ordering;

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
    println!("The secret number is: {_secret_number}");

    
    loop {
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

        // We create a variable named guess. But wait, doesn’t the program already 
        // have a variable named guess? It does, but helpfully Rust allows us to shadow 
        // the previous value of guess with a new one. Shadowing lets us reuse the guess 
        // variable name rather than forcing us to create two unique variables, such as guess_str and guess
        // We bind this new variable to the expression guess.trim().parse(). The guess in 
        // the expression refers to the original guess variable that contained the input 
        // as a string. The trim method on a String instance will eliminate any whitespace 
        // at the beginning and end, which we must do before we can convert the string to 
        // a u32, which can only contain numerical data.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        // A match expression is made up of arms. An arm consists of a pattern to match against, 
        // and the code that should be run if the value given to match fits that arm’s pattern.
        // Rust takes the value given to match and looks through each arm’s pattern in turn.
        match guess.cmp(&_secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // break the loop when the user wins
            }
        }
    }
}
