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

fn get_word_with_guesses(word: &str, guesses: &Vec<char>, new_guess: char) -> (String, bool) {
    let mut word_with_guesses = String::new();
    let mut correct = false;
    for c in word.chars() {
        if guesses.contains(&c) {
            word_with_guesses.push(c);
            if new_guess.eq(&c) {
                correct = true;
            }
        } else {
            word_with_guesses.push('_');
        }
    }
    return (word_with_guesses, correct);
}

fn game_loop(word: &String, mut guesses: Vec<char>) {
    let mut mistakes: u16 = 0;

    loop {
        // println!("{}", get_word_with_guesses(word, &guesses).0);
        println!("Guess a letter:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.len() > 2 {
            println!("Please enter only one character");
            continue;
        }
        guesses.push(input.chars().nth(0).unwrap());
        let test = get_word_with_guesses(word, &guesses, input.chars().nth(0).unwrap());
        if test.0.eq(word) {
            println!("You won!");
            break;
        } else if !test.1 {
           mistakes += 1; 
        } else {
            println!("{}", test.0);
        }
        println!("Mistakes: {}", mistakes);
        if mistakes == 6 {
            println!("You lost!");
            break;
        }
    }
}
