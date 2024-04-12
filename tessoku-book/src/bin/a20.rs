use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            if i >= 1 && j >= 1 && s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]).max(dp[i - 1][j - 1] + 1);
            } else if i >= 1 && j >= 1 {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            } else if i >= 1 {
                dp[i][j] = dp[i - 1][j];
            } else if j >= 1 {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
