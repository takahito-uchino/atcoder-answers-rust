use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: isize,
        k: isize,
        s: Chars,
    }

    let mut set = HashSet::new();

    for _ in 0..m {
        input! {
            x: isize,
            y: isize,
        }
        set.insert((x, y));
    }

    let mut nx = 0;
    let mut ny = 0;

    for i in 0..n {
        let mut dx = 0;
        let mut dy = 0;
        if s[i] == 'R' {
            dx = 1;
        } else if s[i] == 'L' {
            dx = -1;
        } else if s[i] == 'U' {
            dy = 1;
        } else {
            dy = -1;
        }
        nx += dx;
        ny += dy;
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }
        if h < k && set.contains(&(nx, ny)) {
            h = k;
            set.remove(&(nx, ny));
        }
    }

    println!("Yes")
}
