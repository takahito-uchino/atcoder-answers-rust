use std::usize;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut u = usize::MAX;
    let mut d = 0;
    let mut l = usize::MAX;
    let mut r = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                u = u.min(i);
                d = d.max(i);
                l = l.min(j);
                r = r.max(j);
            }
        }
    }

    for i in u..=d {
        for j in l..=r {
            if s[i][j] == '.' {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
