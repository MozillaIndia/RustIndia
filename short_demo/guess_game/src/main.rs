// Task : Guess number game
// Date : 10 Sept 2016
// Author : Vigneshwer

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number game!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("You secret number is {}", secret_number);

    loop {
        println!("Enter the number in your mind");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 =match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}
