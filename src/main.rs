use rand::seq::IteratorRandom;
use std::{
    fs::File,
    io::{BufRead, BufReader, self, Read}
};

const FILENAME: &str = "words.txt";

struct Wordle { 
    word: String,
    bad_chars: Vec<char>,
    good_chars: Vec<char>
}

impl Wordle {
    fn new(word: String) -> Wordle {
        Wordle {
            word: get_random_line_from_file(FILENAME),
            bad_chars: Vec::new(),
            good_chars: Vec::new(),
        }
    }

    fn submit_and_test_guess(&self,guess: String) {
        let guess: Vec<char> = guess.chars().collect();

        let word: Vec<char> = self.word.chars().collect();

        let matching_letters = compare_vec(&guess, &word);
        println!("{:?}",matching_letters)
    }

    fn new_random_word(&mut self) {
        self.word = get_random_line_from_file(FILENAME);

    }
}

fn compare_vec<T: std::cmp::PartialEq >(a: &Vec<T>, b: &Vec<T>) -> Vec<bool> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x == y)
        .collect::<Vec<bool>>()
}

fn get_random_line_from_file(filename: &str) -> String {
    let f = File::open(filename)
        .unwrap_or_else(|e| panic!("Error opening file {}: {}", FILENAME, e));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Error reading line"));

    lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines")
}

fn main() {
    let wordle = Wordle::new(String::from(""));

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = String::from(guess.trim());
        wordle.submit_and_test_guess(guess);
    }

}

