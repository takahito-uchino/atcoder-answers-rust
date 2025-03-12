use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut answer = VecDeque::new();

    for _ in 0..q {
        input! {
            n: usize,
        }

        if n == 1 {
            input! {
                name: String,
            }
            answer.push_back(name);
        } else if n == 2 {
            println!("{}", answer[0]);
        } else if n == 3 {
            answer.pop_front();
        }
    }
}
