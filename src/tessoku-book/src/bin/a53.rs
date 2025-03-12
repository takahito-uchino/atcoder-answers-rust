use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut priority_queue = BinaryHeap::new();

    for _ in 0..q {
        input! {
            n: usize,
        }

        if n == 1 {
            input! {
                price: usize,
            }

            priority_queue.push(Reverse(price));
        } else if n == 2 {
            println!("{}", priority_queue.peek().unwrap().0);
        } else if n == 3 {
            priority_queue.pop();
        }
    }
}
