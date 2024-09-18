use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut boxes = vec![Vec::<usize>::new(); n + 1];
    let mut cards = vec![BTreeSet::<usize>::new(); 200001];

    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            input! {
                i: usize,
                j: usize,
            }
            cards[i].insert(j);
            boxes[j].push(i);
        } else if t == 2 {
            input! {
                i: usize,
            }
            boxes[i].sort_unstable();
            println!("{}", boxes[i].iter().map(|num| num.to_string()).join(" "));
        } else {
            input! {
                i: usize,
            }
            println!("{}", cards[i].iter().map(|num| num.to_string()).join(" "));
        }
    }
}
