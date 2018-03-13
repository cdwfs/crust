// This pulls in the rand crate, and implicitly acts like a "use rand".
extern crate rand;

// Terminology:
// - std is a crate
// - io is a module
// - std::io::StdIn is a struct
// - rand::Rng is a trait, which enables various functions & stuff and
//   I don't really understand it yet.
// If we pull in the io module, std::io below could just be written "io".
// use std::io;

// ...but THIS use is necessary, or else gen_range() is undefined below,
// and I don't really understand why. I need to learn about crates.
use rand::Rng;

use std::cmp::Ordering;

// cargo doc --open generates API docs specific to a project, based on
// its dependencies. Neat! Alas, it does not include the std crate

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1, 101);
    //println!("The secret is {}", secret);

    loop {
        println!("Please input a guess.");
        let mut guess = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // The new declaration will shadow the old one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {} ({} bytes)", guess, bytes);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        };
    }
}
