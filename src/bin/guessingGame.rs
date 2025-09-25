use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Welcome to guessing game!!");

    let secret_num = rand::thread_rng().gen_range(0,101);

    loop {
        println!("Guess a number:");

        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        let guess: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
