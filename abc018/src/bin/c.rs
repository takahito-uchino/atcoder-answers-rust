use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        grid: [String; r],
    }

    let mut memo = vec![vec![0; c + 1]; r];

    for i in 0..r {
        for (j, a) in grid[i].chars().enumerate() {
            if a == 'x' {
                for l in 0..k {
                    if i + 1 < r {
                        memo[i + l][max(0, j as isize - k as isize + 1 + l as isize) as usize] += 1;
                        memo[i + l][min(c, j + k - l)] -= 1;
                    }
                    if i as isize - l as isize >= 0 && l != 0 {
                        memo[i - l][max(0, j as isize - k as isize + 1 + l as isize) as usize] += 1;
                        memo[i - l][min(c, j + k - l)] -= 1;
                    }
                }
            }
        }
    }

    for i in 0..r {
        for j in 1..c {
            memo[i][j] += memo[i][j - 1];
        }
    }

    let mut ans = 0;
    for i in k - 1..r - k + 1 {
        for j in k - 1..c - k + 1 {
            if memo[i][j] == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
