use std::io;
use rand::prelude::*;

const MAX_STRIKES: u8 = 8;


fn main() {
    let words = vec![
        "GOOD".to_string(), 
        "ROCK".to_string(),
        "SHOOT".to_string(),
        "CORRUPTION".to_string(),
        "HOSTAGE".to_string(),
        "COUNCIL".to_string(),
        "REDUCE".to_string(),
        "RELATE".to_string(),
        "TELEVISION".to_string(),
        "SCENE".to_string(),
        "REVIEW".to_string(),
        "THOUGHTFUL".to_string(),
        "VEGETARIAN".to_string(),
        "PERSIST".to_string(),
        "BATTLEFIELD".to_string(),
        "FORGET".to_string(),
        "REALITY".to_string(),
        "ENVIRONMENT".to_string(),
        "JOYFUL".to_string(),
        "RECOMMENDATION".to_string(),
    ];

    let active_word = choose_word(&words);
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut strikes = 0;

    'main: loop {
        clearscreen::clear().expect("Failed to clear screen.");
        println!("\nH A N G   M A N\n\n\n");

        display_word(&active_word, &guessed_letters);

        victory_check(&active_word, &guessed_letters);

        println!("\n\n\nSTRIKES: {}/{}\n", strikes, MAX_STRIKES);

        println!("\nGUESS:");
        let guess = enter_character();

        println!("\nYOU GUESSED: {}\n", guess);

        let result = letter_is_in_word(&active_word, &guess);

        match result {
            true => {
                for c in &guessed_letters {
                    if *c == guess {
                        continue 'main;
                    }
                }
                guessed_letters.push(guess);
            }

            false => {
                strikes += 1;

                if strikes == MAX_STRIKES {
                    clearscreen::clear().expect("Failed to clear screen.");
                    println!("\nYOU LOSE!\n");

                    break;
                }
            },
        }
    }
}

fn choose_word(words: &Vec<String>) -> &String {
    let mut random_word = rand::thread_rng()
        .gen_range(0..words.len()) as usize;

    &words[random_word]
}

fn display_word(active: &String, guessed: &Vec<char>) {
    for c in active.chars() {
        let mut changed = false;

        for l in guessed {
            if c == *l {
                print!("{}", l);

                changed = true;
            }
        }

        if !changed {
            print!("_ ");
        }
    }
}

fn enter_character() -> char {
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read input.");

    let entry: char = match entry.trim().to_uppercase().parse() {
        Ok(c) => c,
        Err(_) => panic!("Invalid input! Type a single character."),
    };

    entry
}

fn letter_is_in_word(word: &String, guess: &char) -> bool {
    for i in word.chars() {
        if i == *guess {
            return true;
        }
    }

    return false;
}

fn victory_check(active: &String, guessed: &Vec<char>) {
    let mut count = 0 as usize;

    for c in active.chars() {
        for l in guessed {
            if *l == c {
                count += 1;
            }
        }
    }
    
    if count == active.len() {
        println!("\nYOU WIN!!!\n");

        std::process::exit(0);
    }
}
