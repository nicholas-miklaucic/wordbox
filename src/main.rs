use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordBox {
    words: Vec<String>,
}

impl WordBox {
    pub fn try_new(words: &[String]) -> Option<WordBox> {
        if is_word_box(words) {
            return Some(WordBox {
                words: words.to_owned(),
            });
        }
        None
    }
}

impl Display for WordBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join("\n"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexicon {
    words: Vec<String>,
}

impl Lexicon {
    /// Get a list of words that start with the given prefix and are of the given length
    fn words_with_prefix(&self, prefix: &String, word_len: usize) -> Vec<String> {
        self.words
            .iter()
            .filter(|word| word.starts_with(prefix) && word.len() == word_len)
            .cloned()
            .collect()
    }
}

impl Display for Lexicon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.words.join(", "))
    }
}

/// Filter out words that contain uppercase letters, punctuation, or whitespace
fn filter_words(filename: &str) -> Vec<String> {
    let file: File = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| {
            line.chars()
                .all(|c| !c.is_uppercase() && !c.is_ascii_punctuation() && !c.is_whitespace())
        })
        .collect()
}

/// Check if the words form a word box
/// Current check requires that the words are all the same length, and that the grid is symmetric across the diagonal
fn is_word_box(words: &[String]) -> bool {
    if !words.iter().all(|word| word.len() == words.len()) {
        return false;
    }

    // Check if the grid is symmetric
    let len = words.len();
    for i in 0..len {
        for j in 0..len {
            if words[i].chars().nth(j) != words[j].chars().nth(i) {
                return false;
            }
        }
    }

    true
}

fn solve_word_box(words: &[String], box_size: usize, lexicon: &Lexicon) -> bool {
    let len = words.len();

    if len == box_size {
        let word_box = WordBox {
            words: words.to_vec(),
        };
        println!("{:}", word_box);
        return true;
    }

    let prefix: String = words
        .iter()
        .map(|word| word.chars().nth(len).unwrap())
        .collect();

    let choices = lexicon.words_with_prefix(&prefix, box_size);

    for choice in choices.iter() {
        let mut new_words = words.to_owned();
        new_words.push(choice.clone());
        let result = solve_word_box(&new_words, box_size, lexicon);
        if result {
            return true;
        }
    }

    false
}
fn main() {
    let words = filter_words("../3esl.txt");
    let lexicon = Lexicon { words };

    for i in 1..=6 {
        let box_size = i;
        println!("Solving word box of size {}...", box_size);
        solve_word_box(&[], box_size, &lexicon);
    }
}
