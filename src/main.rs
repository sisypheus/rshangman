use std::io;

fn main() {
    println!("Welcome to the Rust-based hangman game!");
    let guesses: Vec<char> = Vec::new();
    let word = get_word();
    game_loop(&word, guesses);
}

fn get_word() -> String {
    let resp = reqwest::blocking::get("https://random-word-api.herokuapp.com/word");

    let res = match resp {
        Err(_) => panic!("Could not get word"),
        Ok(word) => word.json::<serde_json::Value>().unwrap(),
    };
    return res[0].as_str().unwrap().to_string();
}

fn fill_word_with_guesses(word: &String, guesses: &Vec<char>, guessed_word: &mut String) {
    guessed_word.clear();
    for c in word.chars() {
        if guesses.contains(&c) {
            guessed_word.push(c);
        } else {
            guessed_word.push('_');
        }
    }
}

fn win_check(guess: &String) -> bool {
    if guess.contains("_") {
        return false;
    }
    return true;
}

fn handle_continue() {
    println!("Would you like to continue? (y/n)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        let word = get_word();
        let guesses: Vec<char> = Vec::new();
        game_loop(&word, guesses);
    } else {
        println!("Thanks for playing!");
        std::process::exit(0);
    }
}

fn check_guess_correctness(guess: char, word: &String, guesses: &Vec<char>, mistakes: &mut u16, guessed_word: &mut String) {
    if word.contains(guess) {
        fill_word_with_guesses(word, guesses, guessed_word);
    } else {
        println!("Incorrect guess!");
        *mistakes += 1;
    }
}

fn check_end_game(mistakes: u16, guess: &String, word: &String) {
    if mistakes == 6 {
        println!("You lost! The word was {}", word);
    } else if win_check(guess) {
        println!("You won!");
    } else {
        return;
    }
    handle_continue();
}

fn game_loop(word: &String, mut guesses: Vec<char>) {
    let mut mistakes: u16 = 0;
    let mut guessed = word.chars().map(|_| '_').collect::<String>();

    loop {
        println!("{}", guessed);
        println!("{} chances left", 6 - mistakes);
        println!("Guess a letter:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.len() > 2 {
            println!("Please enter only one character");
            continue;
        }
        let guess = input.chars().nth(0).expect("Failed to get character");
        guesses.push(guess);
        check_guess_correctness(guess, word, &mut guesses, &mut mistakes, &mut guessed);
        check_end_game(mistakes, &guessed, word);
    }
}