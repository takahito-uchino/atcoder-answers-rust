use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        s: Chars,
    }

    let mut right_min: HashMap<usize, usize> = HashMap::new();
    let mut left_max: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        if s[i] == 'R' {
            if let Some(&l_max) = left_max.get(&xy[i].1) {
                if xy[i].0 < l_max {
                    println!("Yes");
                    return;
                }
            }
        } else {
            if let Some(&r_min) = right_min.get(&xy[i].1) {
                if xy[i].0 > r_min {
                    println!("Yes");
                    return;
                }
            }
        }

        if s[i] == 'R' {
            right_min
                .entry(xy[i].1)
                .and_modify(|r_min| *r_min = (*r_min).min(xy[i].0))
                .or_insert(xy[i].0);
        } else {
            left_max
                .entry(xy[i].1)
                .and_modify(|l_max| *l_max = (*l_max).max(xy[i].0))
                .or_insert(xy[i].0);
        }
    }

    println!("No")
}
