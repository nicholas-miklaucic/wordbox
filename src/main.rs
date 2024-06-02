use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rand::prelude::*;
use rand::seq::SliceRandom;

// use itertools::Itertools;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordBox {
    words: Vec<String>,
}

impl WordBox {
    pub fn try_new(words: &Vec<String>) -> Option<WordBox> {
        if is_word_box(words) {
            return Some(WordBox {
                words: words.clone(),
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

fn pick_random_strings(strings: &Vec<String>, len: usize) -> Option<WordBox> {
    let mut rng = thread_rng();
    WordBox::try_new(&strings.choose_multiple(&mut rng, len).cloned().collect())
}
/*
fn is_word_box(words: &[&String]) -> bool {
    if words.len() != 3 {
        return false;
    }

    for (i, word1) in words.iter().enumerate() {
        for (j, word2) in words.iter().enumerate().skip(i + 1) {
            if word1.len() != 3 || word2.len() != 3 {
                continue;
            }

            let mut is_word_box = true;

            for k in 0..3 {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for l in 0..3 {
                    if word1.chars().nth(l) == word2.chars().nth(k) {
                        row[l] = true;
                        col[k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }
            }

            if is_word_box {
                return true;
            }

            if i != j {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for k in 0..3 {
                    if word1.chars().nth(k) == word2.chars().nth(3 - k) {
                        row[k] = true;
                        col[3 - k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }

                if is_word_box {
                    return true;
                }
            }
        }
    }

    false
}
*/

/*
fn is_word_box(words: &[&String]) -> bool {
    if words.len() != 3 {
        return false;
    }

    for i in 0..3 {
        let word1 = words[i];
        for j in (i + 1)..3 {
            let word2 = words[j];

            if word1.len() != 3 || word2.len() != 3 {
                continue;
            }

            let mut is_word_box = true;

            for k in 0..3 {
                let mut row = [false; 3];
                let mut col = [false; 3];

                for l in 0..3 {
                    if word1.chars().nth(l) == word2.chars().nth(k) {
                        row[l] = true;
                        col[k] = true;
                    }
                }

                if !row.iter().all(|&x| x) || !col.iter().all(|&x| x) {
                    is_word_box = false;
                    break;
                }
            }

            if is_word_box {
                return true;
            }
        }
    }

    false
}
*/

fn is_word_box(words: &[String]) -> bool {
    // Check if there are exactly 3 words and each word is exactly 3 characters long
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

fn main() {
    let len = 3;

    let words = filter_words("3esl.txt");
    let box_words: Vec<_> = words
        .iter()
        .filter(|line| line.len() == len)
        .map(|s| s.to_string())
        .collect();

    let mut random_words = pick_random_strings(&box_words, len);
    while random_words.is_none() {
        random_words = pick_random_strings(&box_words, len);
    }

    println!("{}", random_words.unwrap());
}
