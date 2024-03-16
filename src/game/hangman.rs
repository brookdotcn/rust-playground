use colored::Colorize;
use rand::Rng;
use std::io::{stdin, stdout, Write};

fn clear_console() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

/// iterate over each `char` and print it if guessed, otherwise an underscore.
/// ### args
/// * **word** `&str` the chosen word
/// * **correct_chars** `&Vec<char>` all correct guesses
fn display_word(word: &str, correct_chars: &Vec<char>) -> () {
    for i in 0..word.len() {
        let char = word.chars().nth(i).unwrap();
        if correct_chars.contains(&char) {
            print!("{char}");
            continue;
        }

        print!(" _ ");
    }
}

/// count the matches of a guess within a word.
/// ### args
/// * **word** `&str` the chosen word
/// * **guess** `char` match to find in **word**
fn find_occurences(word: &str, guess: char) -> usize {
    word.matches(guess).collect::<Vec<&str>>().len()
}

/// validate guess is present in a word.
/// ### args
/// * **word** `&str` the chosen word
/// * **guess** `char` match to find in **word**
fn guess_is_valid(word: &str, guess: char) -> bool {
    word.find(guess).is_some()
}

/// randomly select a word from a list.
/// ### args
/// * **words** `[&str]` list of words to use in the game
fn choose_word<'l>(words: &'l [&str; 5]) -> &'l str {
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index]
}

pub fn hangman_run() -> () {
    let words: [&str; 5] = ["rust", "trait", "arc", "impl", "match"];
    let word = choose_word(&words);

    let mut correct_guesses = Vec::<char>::new();
    let mut incorrect_guesses = Vec::<char>::new();
    let mut error = String::new();

    clear_console();

    loop {
        clear_console();

        /* print error after console being cleared */
        if !error.is_empty() {
            println!("[WARNING] {}\n", error.yellow());
        }

        /* handle game result */
        if correct_guesses.len() == word.len() {
            println!("[SUCCESS] the word was {}", word.green());
            break;
        } else if incorrect_guesses.len() >= 5 {
            println!("[FAILURE] the word was {}", word.red());
            break;
        }

        /* show lives and previous incorrect guesses if an incorrect guess has been made */
        if incorrect_guesses.len() > 0 {
            let previous_incorrect = incorrect_guesses
                .iter()
                .map(|char| char.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            print!("[LIVES] {}", 5 - incorrect_guesses.len());
            print!(" - ");
            print!("[INCORRECT] {previous_incorrect}\n\n");
        }

        display_word(&word, &correct_guesses);

        let mut guess = String::new();
        print!(" | [GUESS] ");

        stdout().flush().unwrap();
        stdin().read_line(&mut guess).unwrap();

        if guess.len() > 2 || guess.len() == 1 {
            error = String::from("guess should be a single character");
            continue;
        }

        let guess_char = guess.chars().nth(0).unwrap();
        if !guess_char.is_alphabetic() {
            error = String::from("guess should be alphabetic");
            continue;
        }

        if guess_is_valid(&word, guess_char) {
            for _ in 0..find_occurences(&word, guess_char) {
                correct_guesses.push(guess_char);
            }
        } else {
            if !incorrect_guesses.contains(&guess_char) {
                incorrect_guesses.push(guess_char);
            }
        }

        /* reset error after a successful run, incorrect or correct */
        error.clear();
    }
}
