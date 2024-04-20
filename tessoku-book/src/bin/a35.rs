use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        dp[n][i + 1] = a[i];
    }

    for i in (1..=n - 1).rev() {
        for j in 1..=i {
            if i % 2 == 1 {
                dp[i][j] = max(dp[i + 1][j], dp[i + 1][j + 1]);
            } else {
                dp[i][j] = min(dp[i + 1][j], dp[i + 1][j + 1]);
            }
        }
    }

    println!("{}", dp[1][1])
}
