use rand::seq::IteratorRandom;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
};

const FILENAME: &str = "words.txt";

#[derive(PartialEq, Debug)]
enum LetterMatch {
    Belongs,
    NotInWord,
    BelongsElsewhere,
}

enum GameState {
    Guessing,
    Lost,
    Won,
}

struct Wordle {
    word: String,
    bad_chars: Vec<char>,
    good_chars: Vec<char>, //TODO ADD LOGIC TO USE LETTERMATCH INSTEAD OF BOOLS
    guesses_map: HashMap<u32, (String, Vec<LetterMatch>)>,
    game_state: GameState,
    number_of_guesses: u32,
}

impl Wordle {
    fn new() -> Wordle {
        Wordle {
            word: get_random_line_from_file(FILENAME),
            bad_chars: Vec::new(),
            good_chars: Vec::new(),
            guesses_map: HashMap::new(),
            game_state: GameState::Guessing,
            number_of_guesses: 5,
        }
    }

    fn submit_and_test_guess(&mut self, guess: String) {
        let vec_guess: Vec<char> = guess.chars().collect();

        let vec_word: Vec<char> = self.word.chars().collect();

        let matching_letters = compare_vec(&vec_guess, &vec_word);

        if !matching_letters.contains(&LetterMatch::NotInWord)
            && !matching_letters.contains(&LetterMatch::BelongsElsewhere)
            && guess.len() == 5
        {
            self.game_state = GameState::Won
        }

        self.guesses_map.insert(self.guesses_map.len() as u32, (guess, matching_letters));

        vec_guess.iter().for_each(|c| {
            if vec_word.contains(c) && (!self.good_chars.contains(c)) {
                self.good_chars.push(*c)
            }
            if !vec_word.contains(c) && (!self.bad_chars.contains(c)) {
                self.bad_chars.push(*c)
            }
        });
    }

    fn new_random_word(&mut self) {
        self.word = get_random_line_from_file(FILENAME);
    }
}

fn compare_vec<T: std::cmp::PartialEq>(a: &Vec<T>, b: &Vec<T>) -> Vec<LetterMatch> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| {
            if x == y {
                LetterMatch::Belongs
            } else if (x != y) && (!b.contains(x)) {
                LetterMatch::NotInWord
            } else {
                LetterMatch::BelongsElsewhere
            }
        })
        .collect::<Vec<LetterMatch>>()
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
    println!("{}", wordle.word);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess = String::from(guess.trim());
        wordle.submit_and_test_guess(guess);
        println!("{:#?}", wordle.guesses_map)
    }
}
