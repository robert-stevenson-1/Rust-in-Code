use std::io; // import io library into scope
use rand::Rng; // import rand library into scope
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
        // generate a random number
        // `thread_rng` creates a random number generator
        // `gen_range` generates a random number in the range
        // `1..=100` is the range from 1 to 100, inclusive
        let rand_num = rand::thread_rng().gen_range(1..=100);
    
        println!("The secret number is: {rand_num}");
    
    // `loop` is used to create an infinite loop
    loop{
        println!("Please input your guess.");
        
        // mutable variable to store user input
        // `let` creates the variable 
        // `mut` makes the variable mutable, allowing it to be changed
        let mut guess = String::new(); // create mutable variable `guess` with a new instance of the `String` type
        
        // read user input
        // this is equivalent to `std::io::stdin().read_line(&mut guess).expect("Failed to read line");`
        io::stdin()
        // reads user input into the mutable variable reference (reference are immutable by default, hence `&mut`)
        .read_line(&mut guess) // This returns a `Result` type, which is a type that represents either success (`Ok`) or failure (`Err`).
        // This is error handling on the `Result` type. 
        // If the `Result` type is `Ok`, then the expression will be executed. 
        // If the `Result` type is `Err`, then the expression will be executed and the `Err` will be returned.
        .expect("Failed to read line"); // this method will panic and crash the program if an error occurs
        // error handling safely will be in chapter 9
        
        // print user input
        println!("You guessed: {guess}");
        
        // rust allows variable to shadow another variable with the same name
        let guess: u32 = match guess // convert user input to integer
        .trim() // trim the user input, eliminating any whitespace at the start and end of the string
        .parse() // convert the string to integer
        {
            // The value is a valid number, it's `ok`, continues the program logic
            Ok(num) => num,
            // The value it `err`, then continue the loop from the beginning
            Err(_) => continue,
        };

        // compare the user input with the random number
        // `cmp` compares two values and returns an `Ordering
        // `match` is used to decide what to do next based on the variant of `Ordering` that is true
        match guess.cmp(&rand_num) {
            // compare the user input with the random number
            // `Ordering` is an enum that can be either `Less`, `Greater`, or `Equal`
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break the loop
            }
        }
    }
} 
