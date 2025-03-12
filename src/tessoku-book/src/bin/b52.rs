use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    }

    let mut queue = VecDeque::new();
    queue.push_back(x - 1);
    a[x - 1] = '@';

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if pos > 0 &&a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos + 1 < n && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }

    println!("{}", a.into_iter().collect::<String>());
}
