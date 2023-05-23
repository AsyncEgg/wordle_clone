use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{self, BufRead, BufReader}, collections::HashMap,
};

const FILENAME: &str = "words.txt";

struct Wordle {
    word: String,
    bad_chars: Vec<char>,
    good_chars: Vec<char>,
    where_chars_go: HashMap<u32, Vec<char>>
}

impl Wordle {
    fn new() -> Wordle {
        Wordle {
            word: get_random_line_from_file(FILENAME),
            bad_chars: Vec::new(),
            good_chars: Vec::new(),
            where_chars_go: HashMap::new(),
        }
    }

    fn submit_and_test_guess(&mut self, guess: String) {
        let guess: Vec<char> = guess.chars().collect();

        let word: Vec<char> = self.word.chars().collect();

        let matching_letters = compare_vec(&guess, &word);

        println!("{:?}", matching_letters);

        guess.iter().for_each(|c |{
            if word.contains(c) && (!self.good_chars.contains(c)) {
                self.good_chars.push(*c)
            }  
            if !word.contains(c) && (!self.bad_chars.contains(c)) {
                self.bad_chars.push(*c)
            }
        });

        println!("Good: {:?}\nBad: {:?}",self.good_chars,self.bad_chars);
        
        if !matching_letters.contains(&false) && guess.len() == 5 {
            println!("you win!")
        }
    }

    fn new_random_word(&mut self) {
        self.word = get_random_line_from_file(FILENAME);
    }
}

fn compare_vec<T: std::cmp::PartialEq>(a: &Vec<T>, b: &Vec<T>) -> Vec<bool> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x == y)
        .collect::<Vec<bool>>()
}

fn get_random_line_from_file(filename: &str) -> String {
    let f =
        File::open(filename).unwrap_or_else(|e| panic!("Error opening file {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Error reading line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn main() {
    let mut wordle = Wordle::new();
    println!("{}",wordle.word);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess = String::from(guess.trim());
        wordle.submit_and_test_guess(guess);

    }
}
