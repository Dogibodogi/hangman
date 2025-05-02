use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
fn main() -> io::Result<()> {
    let topic = choose_topic();
    let mut topic_as_string = String::new();
    match topic {
        Ok(s) => topic_as_string = s,
        Err(e) => eprintln!("Error: {}", e),
    };
    let word = choose_word(topic_as_string);

    match word {
        Ok(s) => start_game(s),
        Err(_) => (),
    }
    Ok(())
}

fn choose_topic() -> io::Result<String> {
    println!("Welcome to hangman!Please choose the category of the word!");
    println!(" 1.Animals \n 2.Fruits \n 3.Countries");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error at reading from keyboard!");
    let choice = buf.trim();
    let topic = match choice {
        "1" => "animal",
        "2" => "fruit",
        "3" => "country",
        _ => {
            println!("Invalid Option!");
            return Ok("Error".to_string());
        }
    };
    return Ok(topic.to_string());
}

fn choose_word(topic_as_string: String) -> io::Result<String> {
    let path_buf: PathBuf = PathBuf::from("C://Users//Sacal//Desktop//test//test1.txt");
    let file = File::open(&path_buf)?;
    let reader = BufReader::new(file);
    let mut start_number = 0;
    let mut end_number = 0;
    for (line_number, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(s) => {
                if let Some(word) = s.split_once(':') {
                    if word.0.trim() == topic_as_string.trim() && start_number == 0 {
                        start_number = line_number;
                    } else if word.0.trim() == topic_as_string.trim() {
                        end_number = line_number;
                    }
                }
            }
            Err(e) => eprintln!("Error:{}", e),
        }
    }

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);
    let word_count = end_number - start_number;
    let random_word_line = random_number % word_count;

    let file = File::open(&path_buf)?;
    let reader = BufReader::new(file);

    for (index, line_result) in reader.lines().enumerate() {
        if index == random_word_line + start_number {
            match line_result {
                Ok(s) => {
                    if let Some(word) = s.split_once(':') {
                        return Ok(word.1.trim().to_string());
                    }
                }
                Err(_) => return Ok("Error".to_string()),
            }
        }
    }
    Ok("Word not found".to_string())
}

fn start_game(word: String) {
    let mut string: String = word
        .chars()
        .map(|ch| if ch.is_alphabetic() { '_' } else { ch })
        .collect();
    println!("{string}");
    let mut buf = String::new();
    let mut tries = 3;
    loop {

        if string==word{
            println!("You won!");
            break;
        }
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("Error while reading from keyboard");
        let maybe_character = buf.trim().chars().next();
        let mut ch = '0';
        let mut changed = false;
        match maybe_character {
            Some(c) => ch = c,
            None => (),
        }
        let mut chars: Vec<char> = string.chars().collect();

        for (number, letter) in word.chars().enumerate() {
            
            if letter == ch && chars[number] != letter {
                chars[number] = letter;
                changed = true;
            }
        }
        if changed == true {
            let updated_string: String = chars.into_iter().collect();
            string = updated_string.clone();
            println!("{updated_string}");
        } else if tries>0 {
            tries = tries - 1;
            let updated_string: String = chars.into_iter().collect();
            println!("Letter does not exist in word! {tries} tries left!");
            println!("{updated_string}");
        } else if tries==0{
            println!("Game over! The word was : {word}");
            break;
        } 


    }
}
