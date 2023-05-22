pub struct Wordle { 
    word: &'static str,
    board: Vec<Vec<&'static str>>,
    input: &'static str,
}

impl Wordle {
    pub fn new(word: &'static str, board: Vec<Vec<&'static str>>, input: &'static str) -> Wordle {
        Wordle {
            word,board,input
        }
    }

    pub fn grab_random_word() -> &'static str {
        ""
    }
}

