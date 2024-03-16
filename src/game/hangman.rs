use colored::Colorize;
use rand::Rng;
use std::io::{stdin, stdout, Write};

fn choose_word<'l>(words: &'l [&str; 5]) -> &'l str {
    let index = rand::thread_rng().gen_range(0..words.len());
    words[index]
}

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn game_is_over(word: &str, correct: &Vec<char>, incorrect: &Vec<char>) -> bool {
    if correct.len() == word.len() {
        println!("[SUCCESS] the word was {}", word.green());
        return true;
    } else if incorrect.len() >= 5 {
        println!("[FAILURE] the word was {}", word.red());
        return true;
    }

    false
}

fn show_metadata(incorrect: &Vec<char>) -> () {
    let previous_incorrect = incorrect
        .iter()
        .map(|char| char.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    print!("[LIVES] {}", 5 - incorrect.len());
    print!(" - ");
    print!("[INCORRECT] {previous_incorrect}\n\n");
}

fn display_word(word: &str, correct_chars: &Vec<char>) -> () {
    for i in 0..word.len() {
        let char = word.chars().nth(i).unwrap();
        if correct_chars.contains(&char) {
            print!("{char}");
            continue;
        }

        print!(" _ ");
    }

    print!(" | [GUESS] ");
}

fn judge_guess(word: &str, guess: char, correct: &mut Vec<char>, incorrect: &mut Vec<char>) -> () {
    if word.find(guess).is_some() {
        for _ in 0..word.matches(guess).collect::<Vec<&str>>().len() {
            correct.push(guess);
        }
    } else {
        if !incorrect.contains(&guess) {
            incorrect.push(guess);
        }
    }
}

pub fn hangman_run() -> () {
    let words: [&str; 5] = ["rust", "trait", "arc", "impl", "match"];
    let word = choose_word(&words);

    let mut guess = String::new();
    let mut correct_guesses = Vec::<char>::new();
    let mut incorrect_guesses = Vec::<char>::new();
    let mut error = String::new();

    loop {
        clear();
        guess.clear();

        if !error.is_empty() {
            println!("[WARNING] {}\n", error.yellow());
        }

        if game_is_over(&word, &correct_guesses, &incorrect_guesses) {
            break;
        }

        if !incorrect_guesses.is_empty() {
            show_metadata(&incorrect_guesses);
        }

        display_word(&word, &correct_guesses);

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

        judge_guess(
            word,
            guess_char,
            &mut correct_guesses,
            &mut incorrect_guesses,
        );

        error.clear();
    }
}
