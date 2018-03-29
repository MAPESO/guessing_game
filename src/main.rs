extern crate rand;
extern crate colored;
use rand::{Rng, thread_rng};
use colored::*;
// Native
use std::io;
use std::cmp::Ordering;

fn main() {
    let secret_number: u32 = thread_rng().gen_range(1, 100);

    let mut user = String::new();

    println!("Player's name");

    io::stdin().read_line(&mut user)
        .expect("Failed to read line");

    println!("{}", "Guess the number!!".yellow());

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{} {}", "You have won".green(), user.green());
                break;
            }
        }
    }
}