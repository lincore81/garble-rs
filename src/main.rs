// TODO: Handle multiple identical letters correctly.

mod logic;

use colored::Colorize;
use logic::{evaluate_guess, pick_word, stringify_guess_result, GuessResult};
use std::{
    collections::HashSet,
    io::{self, Write},
};

const MAX_ATTEMPTS: u32 = 5;

fn main() {
    let solutions = include_str!("solutions.in");
    let guesses = include_str!("guesses.in")
        .lines()
        .collect::<HashSet<&str>>();
    let word = pick_word(solutions);

    println!();
    println!("Welcome to Garble!");
    println!("==================");
    println!();
    game_loop(&word, &guesses)
}

fn game_loop(word: &str, allow_list: &HashSet<&str>) {
    let mut attempt = 1;
    while attempt <= MAX_ATTEMPTS {
        let guess = read_user_guess(attempt, MAX_ATTEMPTS);
        if let Err(msg) = logic::validate_guess(word.len(), &guess, allow_list) {
            println!("{}", msg);
            continue;
        }
        let result = evaluate_guess(&guess, word);
        println!("{}", stringify_guess_result(&result));
        if matches!(result, GuessResult::Correct(_)) {
            println!("You did it in {attempt} attempts, congrats!");
            return;
        }
        attempt += 1;
    }
    println!(
        "The correct word was {}, better luck next time!",
        word.green()
    )
}

fn read_user_guess(attempt: u32, max_attempts: u32) -> String {
    let mut guess = String::new();
    print!(
        "{}{}{}{} ",
        attempt.to_string().blue(),
        "/".white(),
        max_attempts.to_string().blue(),
        ">?".white()
    );
    io::stdout().flush().expect("Can't flush stdout.");
    io::stdin()
        .read_line(&mut guess)
        .expect("I don't know what you did, but I can't handle it.");
    guess.trim().to_uppercase()
}
