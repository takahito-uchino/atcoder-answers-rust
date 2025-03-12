use std::cmp::Ordering;

use proconio::input;

fn main() {
    input! {
        x: String,
        n: usize,
        words: [String; n],
    }

    let mut position = vec![0; 26];

    for (i, c) in x.chars().enumerate() {
        position[(c as usize) - ('a' as usize)] = i;
    }

    let mut converted_words: Vec<Vec<usize>> = Vec::new();

    for word in words {
        let mut converted_word: Vec<usize> = Vec::new();
        for c in word.chars() {
            converted_word.push(position[(c as usize) - ('a' as usize)]);
        }
        converted_words.push(converted_word);
    }

    converted_words.sort_by(|a, b| {
        for (x, y) in a.iter().zip(b.iter()) {
            match x.cmp(y) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }
        a.len().cmp(&b.len())
    });

    for word in converted_words {
        for &w in &word {
            print!("{}", x.chars().nth(w).unwrap());
        }

        println!();
    }
}
