use colored::Colorize;
use rand::seq::IteratorRandom;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharMatch {
    Correct(char),
    Occuring(char),
    NotOccuring,
}

#[derive(Debug)]
pub enum GuessResult {
    Correct(Vec<CharMatch>),
    Incorrect(Vec<CharMatch>),
}

pub fn pick_word(solutions: &str) -> String {
    solutions
        .lines()
        .choose(&mut rand::thread_rng())
        .expect("File has no lines?!")
        .to_string()
}

fn stringify_char_result(result: &CharMatch) -> String {
    match result {
        CharMatch::Correct(c) => c.to_string().to_uppercase().green().to_string(),
        CharMatch::Occuring(c) => c.to_string().to_lowercase().yellow().to_string(),
        CharMatch::NotOccuring => String::from('_'),
    }
}

pub fn stringify_guess_result(result: &GuessResult) -> String {
    match result {
        GuessResult::Correct(chars) | GuessResult::Incorrect(chars) => chars
            .iter()
            .map(stringify_char_result)
            .collect::<Vec<String>>()
            .join(" ")
            .to_string(),
    }
}

pub fn validate_guess<'a>(
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

pub fn evaluate_guess(guess: &str, solution: &str) -> GuessResult {
    let mut solution_char_count = solution.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let mut result: Vec<CharMatch> = guess
        .chars()
        .zip(solution.chars())
        .map(|(g_char, s_char)| {
            if g_char == s_char {
                *solution_char_count.get_mut(&s_char).unwrap() -= 1;
                CharMatch::Correct(g_char)
            } else {
                CharMatch::NotOccuring
            }
        })
        .collect();

    result = result
        .into_iter()
        .enumerate()
        .map(|(i, res)| {
            if let CharMatch::NotOccuring = res {
                let g_char = guess.chars().nth(i).unwrap();
                if let Some(count) = solution_char_count.get_mut(&g_char) {
                    if *count > 0 {
                        *count -= 1;
                        return CharMatch::Occuring(g_char);
                    }
                }
                CharMatch::NotOccuring
            } else {
                res
            }
        })
        .collect();

    if result.iter().all(|x| matches!(x, CharMatch::Correct(_))) {
        GuessResult::Correct(result)
    } else {
        GuessResult::Incorrect(result)
    }
}
