use regex::Regex;
use std::io;
use rand::seq::SliceRandom;
mod corpus;

#[derive(Debug)]
enum GuessType {
    WORD,
    LETTER,
    NONE,
}

#[derive(PartialEq)]
enum GameStatus {
    WON,
    LOST,
    PLAYING
}

fn main() {
    let &word = corpus::CORPUS.choose(&mut rand::thread_rng()).unwrap();
    let alphabet_regex = Regex::new(r".").unwrap();
    let hidden_word = alphabet_regex.replace_all(word, "_");
    let mut hidden_split_word:Vec<&str> = hidden_word.split("").collect();

    let max_tries = 6;
    let mut current_tries = 0;

    let mut game_status = GameStatus::PLAYING;

    println!("Bienvenido al juego del ahorcado, tienes 6 intentos para adivinar la palabra oculta. ¿Te atreves?");
    println!("Esta es la palabra que tienes que adivinar:");
    println!("{}", hidden_word);

    loop {
        let mut guess = String::new();

        println!("Introduce una letra o una palabra");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        guess = String::from(guess.trim());

        let guess_type = match guess.len() {
            0 => GuessType::NONE,
            1 => GuessType::LETTER,
            _ => GuessType::WORD,
        };

        match guess_type {
            GuessType::LETTER => {
                if word.contains(&guess) {
                    println!("¡Correcto! la letra {} forma parte de la palabra. ¡Sigue así!", guess);
                    let split_word:Vec<&str> = word.split("").collect();
                    for i in 0..split_word.len() {
                        if split_word[i] == guess {hidden_split_word[i] = split_word[i]}
                    }

                    if hidden_split_word.join("").trim() == String::from(word) {
                        game_status = GameStatus::WON;
                        break;
                    } else {
                        println!("Lo que llevas descubierto de la palabra es: {:?}", hidden_split_word.join(""));
                    }

                } else {
                    current_tries += 1;
                    println!("¡Has fallado! Te quedan {} intentos.", max_tries - current_tries);
                }
            },
            GuessType::WORD => {
                if word == &guess {
                    game_status = GameStatus::WON;
                    break;
                } else {
                    current_tries += 1;
                    println!("¡Has fallado! Te quedan {} intentos.", max_tries - current_tries);
                }
            },
            _ => {
                println!("No has introducido nada, inténtalo de nuevo.")
            }
        }

        if max_tries == current_tries {
            game_status = GameStatus::LOST;
            break;
        };
    }

    if game_status == GameStatus::LOST {
        println!("¡Has perdido!")
    } else {
        println!("¡ENHORABUENA!¡HAS GANADO! La palabra era {}", word);
    }

}
