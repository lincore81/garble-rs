// TODO: Handle multiple identical letters correctly.

use colored::Colorize;
use rand::seq::IteratorRandom;
use std::{
    collections::HashSet,
    io::{self, Write},
};

const MAX_ATTEMPTS: u32 = 5;

#[derive(Debug)]
enum CharGuessResult {
    Correct(char),
    Occuring(char),
    NotOccuring,
}

#[derive(Debug)]
enum GuessResult {
    Correct,
    Incorrect(Vec<CharGuessResult>),
}

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
        if let Err(msg) = validate_guess(word.len(), &guess, allow_list) {
            println!("{}", msg);
            continue;
        }
        let result = evaluate_guess(&guess, word);
        println!(
            "{}",
            stringify_guess_result(&result, word, attempt, MAX_ATTEMPTS)
        );
        if matches!(result, GuessResult::Correct) {
            return;
        }
        attempt += 1;
    }
}

fn pick_word(solutions: &str) -> String {
    solutions
        .lines()
        .choose(&mut rand::thread_rng())
        .expect("File has no lines?!")
        .to_string()
}

fn validate_guess<'a>(
    word_length: usize,
    guess: &'a str,
    allow_list: &HashSet<&str>,
) -> Result<&'a str, String> {
    // println!(
    //     "guess: '{}', word_length: {}, guess.len: {}",
    //     guess,
    //     word_length,
    //     guess.len()
    // );
    let is_correct_length = guess.len() == word_length;
    let is_in_list = allow_list.contains(guess);
    if !is_correct_length {
        return Result::Err(format!("Your guess must be {} letters long.", word_length));
    }
    if !is_in_list {
        return Result::Err("Your guess must be an English word.".to_string());
    }
    Result::Ok(guess)
}

fn read_user_guess(attempt: u32, max_attempts: u32) -> String {
    let mut guess = String::new();
    print!(
        "{}{}{} {} ",
        attempt.to_string().blue(),
        "/".blue(),
        max_attempts.to_string().blue(),
        ">?".blue()
    );
    io::stdout().flush().expect("Can't flush stdout.");
    io::stdin()
        .read_line(&mut guess)
        .expect("I don't know what you did, but I can't handle it.");
    guess.trim().to_uppercase()
}

fn stringify_char_result(result: &CharGuessResult) -> String {
    match result {
        CharGuessResult::Correct(c) => c.to_string().to_uppercase().green().to_string(),
        CharGuessResult::Occuring(c) => c.to_string().to_lowercase().yellow().to_string(),
        CharGuessResult::NotOccuring => String::from('_'),
    }
}

fn stringify_guess_result(
    result: &GuessResult,
    word: &str,
    attempts: u32,
    max_attempts: u32,
) -> String {
    match result {
        GuessResult::Incorrect(chars) if attempts < max_attempts => chars
            .iter()
            .map(stringify_char_result)
            .collect::<Vec<String>>()
            .join(" ")
            .to_string(),
        GuessResult::Correct => format!("Correct! You did it in {attempts} attempts."),
        _ => format!(
            "The correct word was {}, better luck next time.",
            word.green()
        ),
    }
}

fn evaluate_guess(guess: &str, word: &str) -> GuessResult {
    let zipped = guess.chars().zip(word.chars());
    // apparently iter.all below consumes the iter and that's a mutation.
    // So I create a Vec instead and then an iter from that in the next line.
    let char_results: Vec<CharGuessResult> =
        zipped.map(|(a, b)| evaluate_char(a, b, word)).collect();
    let is_correct = char_results
        .iter()
        .all(|result| matches!(result, CharGuessResult::Correct(_)));
    if is_correct {
        GuessResult::Correct
    } else {
        GuessResult::Incorrect(char_results)
    }
}

fn evaluate_char(guess_char: char, word_char: char, word: &str) -> CharGuessResult {
    if guess_char == word_char {
        CharGuessResult::Correct(guess_char)
    } else if word.contains(guess_char) {
        CharGuessResult::Occuring(guess_char)
    } else {
        CharGuessResult::NotOccuring
    }
}
