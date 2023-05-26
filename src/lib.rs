use rand::seq::IteratorRandom;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "words.txt";

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum LetterMatch {
    Belongs,
    NotInWord,
    BelongsElsewhere,
}

#[derive(PartialEq)]
pub enum GameState {
    Guessing,
    Lost,
    Won,
}

pub struct Wordle {
    pub word: String,
    pub perfect_guess: Vec<char>,
    pub bad_guess: Vec<char>,
    pub good_guess: Vec<char>, //TODO ADD LOGIC TO USE LETTERMATCH INSTEAD OF BOOLS
    pub guesses_map: BTreeMap<u32, (String, Vec<LetterMatch>)>,
    pub game_state: GameState,
    max_number_of_guesses: u32,
}

impl Default for Wordle {
    fn default() -> Self {
        Wordle {
            word: get_random_line_from_file(FILENAME),
            perfect_guess: Vec::new(),
            bad_guess: Vec::new(),
            good_guess: Vec::new(),
            guesses_map: BTreeMap::new(),
            game_state: GameState::Guessing,
            max_number_of_guesses: 5,
        }
    }
}

impl Wordle {
    pub fn submit_and_test_guess(&mut self, guess: String) {
        match self.max_number_of_guesses {
            0 => self.game_state = GameState::Lost,
            _ => self.max_number_of_guesses -= 1,
        }
        let vec_guess: Vec<char> = guess.chars().collect();

        let vec_word: Vec<char> = self.word.chars().collect();

        let matching_letters = compare_vec(&vec_guess, &vec_word);

        if !matching_letters.contains(&LetterMatch::NotInWord)
            && !matching_letters.contains(&LetterMatch::BelongsElsewhere)
            && guess.len() == 5
        {
            self.game_state = GameState::Won
        }

        self.guesses_map
            .insert(self.guesses_map.len() as u32, (guess, matching_letters.clone()));

        vec_guess.iter().enumerate().for_each(|(index,c)| {
            if vec_word.contains(c) && (!self.good_guess.contains(c)) {
                self.good_guess.push(*c)
            }
            if !vec_word.contains(c) && (!self.bad_guess.contains(c)) {
                self.bad_guess.push(*c)
            } 
            if matching_letters[index] == LetterMatch::Belongs {
                self.perfect_guess.push(*c)
            }
        });
    }

    pub fn new_random_word(&mut self) {
        self.word = get_random_line_from_file(FILENAME);
        self.bad_guess = Vec::new();
        self.good_guess = Vec::new();
        self.guesses_map = BTreeMap::new();
        self.game_state = GameState::Guessing;
        self.max_number_of_guesses = 5;
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
