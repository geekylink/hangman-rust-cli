use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

static MIN_SIZE: usize = 3;

fn guessWord(word: &String, letters: &String) -> String {
    // returns word, builds it one char at a time, checks if char has been guessed
    let mut retWord = String::new();


    for mut c in word.chars() {
        match letters.find(c) {
            None => {
                retWord.push_str("_ ");
            }
            _ => {
                retWord.push(c);
                retWord.push(' ');
            }
        };

    }
    retWord
}

fn getWordFromFile(fileName: &str) -> String {

    let file = File::open(fileName).unwrap();
    let reader = BufReader::new(file);
    let mut v: Vec<String> = vec![];

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.len() >= MIN_SIZE { 
            v.push(line);
        }
    }

    let r = rand::thread_rng().gen_range(0, v.len());

    v[r].to_string()
}

fn pickFile() -> String {
    let mut fileName = String::new();

    loop {
        println!("Pick difficulty:");
        println!("1 - Easy");
        println!("2 - Medium");
        println!("3 - Hard");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
                   .expect("Failed to read line");

        match choice.to_lowercase().trim() {
            "1" | "easy" => {
                fileName = "words-easy.txt".to_string();
                break;
            }
            "2" | "medium" => {
                fileName = "words-medium.txt".to_string();
                break;
            }
            "3" | "hard" => {
                fileName = "words-hard.txt".to_string();
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        };
    }

    fileName
}

fn main() {
    println!("Hangman game");

    let fileName = pickFile();

    let word = getWordFromFile(&fileName);
    let mut letters = String::new();
    let mut numGuess: u8 = 9;

    loop {
        let wordWithLets = guessWord(&word, &letters);

        printMan(numGuess);
        println!("The word is:");
        println!("{}",wordWithLets);

        if !wordWithLets.contains("_") {
            println!("You won the game!");
            return;
        }

        if letters.len() > 0 {
            print!("Letters tried: ");

            for c in letters.chars() {
                print!("{} ", c);
            }
            println!("");
        }

        println!("You have {} guesses left. Choose carefully: ", numGuess);

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                   .expect("Failed to read line");

        println!("");

        let guess: char = match guess.trim().parse() {
            Ok(c) => c,
            Err(_) => {
                println!("Not a letter");
                continue;
            }
        };

        if letters.contains(guess) {
            println!("You already guessed that!");
            continue;
        }

        if word.contains(guess) {
            println!("Lucky guess!");
        }
        else {
            println!("Wrong! One step closer to the noose.");
            numGuess -= 1;
        }

        if numGuess == 0 {
            println!("Haha you lose!");
            printMan(numGuess);
            println!("The word was: {}", word);
            break;
        }

        letters.push_str(&guess.to_string());
    }
}

fn printMan(numGuess: u8) {
    match numGuess { 
        9 => {
            println!(" _____");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        8 => {
            println!(" _____");
            println!("|     `");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
            println!("The rope has been hung!");
        }
        7 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        6 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|     |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        5 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|    \\|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        4 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|    \\|/");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        3 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|    \\|/");
            println!("|     |");
            println!("|");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        2 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|    \\|/");
            println!("|     |");
            println!("|     |");
            println!("|");
            println!("|");
            println!("|");
            println!("===============");
        }
        1 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|    \\|/");
            println!("|     |");
            println!("|     |");
            println!("|    /");
            println!("|");
            println!("|");
            println!("===============");
        }
        0 => {
            println!(" _____");
            println!("|     `");
            println!("|     O");
            println!("|     |");
            println!("|    /|\\");
            println!("|     |");
            println!("|    /\\");
            println!("|");
            println!("|");
            println!("===============");
        }
        _ => {
            println!("lol no pic");
        }
    }
    
}
