extern crate rand;

use std::{io, cmp};
use rand::{thread_rng, Rng};
use cmp::Ordering;

fn main() {
    println!("Guess the number!\n");
    let secret = thread_rng().gen_range(1, 101);

    let mut is_match = false;
    while !is_match {
        let guess: u32 = ask_number();
        is_match = match_input(guess, secret);
    }
}

fn match_input (guess: u32, secret: u32) -> bool {
    match guess.cmp(&secret) {
        Ordering::Less    => println!("higher!"),
        Ordering::Greater => println!("lower!"),
        Ordering::Equal   => println!("bingo! 🎉"),
    }
    return guess.eq(&secret);
}

fn ask_number () -> u32 {
    println!("Please input your guess : ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();

    // parse to a number
    let guess: u32 = guess.trim().parse()
        .expect("You need to input a number :(");

    return guess;
}
