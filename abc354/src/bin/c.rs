use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        cards: [(usize, usize); n],
    }

    let mut indexed_cards: Vec<(usize, (usize, usize))> =
        cards.iter().enumerate().map(|(i, &v)| (i, v)).collect();

    indexed_cards.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));

    let mut answer = Vec::new();

    answer.push(indexed_cards[0].0 + 1);

    for i in 1..n {
        if indexed_cards[i - 1].1 .1 > indexed_cards[i].1 .1 {
            answer.push(indexed_cards[i].0 + 1)
        }
    }

    answer.sort_unstable();

    println!("{}", answer.len());
    println!("{}", answer.iter().map(|&num| num.to_string()).join(" "));
}
