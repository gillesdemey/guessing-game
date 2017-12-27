extern crate rand;

use std::{io, cmp};
use rand::{thread_rng, Rng};
use cmp::Ordering;

fn main() {
    println!("Guess the number!\n");

    let secret = thread_rng().gen_range(1, 101);

    loop {
        let guess: u32 = ask_number();

        if match_input(guess, secret) {
            break;
        }
    }
}

fn match_input (guess: u32, secret: u32) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less    => println!("higher!"),
        Ordering::Greater => println!("lower!"),
        Ordering::Equal   => println!("bingo! 🎉"),
    }

    guess.eq(&secret)
}

fn ask_number () -> u32 {
    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        // parse to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };

        return guess;
    }
}
