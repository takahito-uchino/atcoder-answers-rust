use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut points = Vec::new();
    let mut points_set = HashSet::new();

    let mut x = 0;
    let mut y = 0;
    points.push((x, y));
    points_set.insert((x, y));

    for c in s {
        if c == 'R' {
            x += 1;
        } else if c == 'L' {
            x -= 1;
        } else if c == 'U' {
            y += 1;
        } else if c == 'D' {
            y -= 1;
        }
        points.push((x, y));
        points_set.insert((x, y));
    }

    if points.len() != points_set.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
