use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, usize); n],
    }

    let mut used = vec![false; n];
    let mut answer = 0;

    for i in 1..=d {
        let mut max_value = 0;
        let mut max_id = usize::MAX;

        for j in 0..n {
            if !used[j] && max_value < xy[j].1 && xy[j].0 <= i {
                max_value = xy[j].1;
                max_id = j;
            }
        }

        if max_id != usize::MAX {
            answer += max_value;
            used[max_id] = true;
        }
    }

    println!("{}", answer);
}
