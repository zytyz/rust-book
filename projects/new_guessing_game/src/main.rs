use std::io;
use rand::Rng;
use std::cmp::Ordering; // An enum type with variants Less, Greater, Equal

fn main() {
    println!("Guess the number!");


    // thread_rng(): local to current thread
    // gen_range() takes in a range expression: start..=end
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        

        // variables are immutable by default
        let mut guess = String::new();

        io::stdin()
            // "&" indicates a reference to guess. Immutable by default.
            .read_line(&mut guess)
            // read_line() returns a "Result" type, which is an enum
            // Result has variants ok or Err.
            // Result has a method expect().
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        // Shadow the previous guess variable so we can reuse the name "guess" for another type

        let guess: u32 = match guess.trim().parse() {
            // Ok contains the result num.
            // if parse() returns a match to Ok(num), then this would be called.
            Ok(num) => num,
            // This matches any Err(_) value, no matter the information inside them.
            Err(_) => continue,
        };

        
        // cmp(): returns an Ordering variant. Types have to match.
        // match: decide what to do with the result
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
