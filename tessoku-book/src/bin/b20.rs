use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![20000; m + 1]; n + 1];

    dp[0][0] = 0;

    for i in 0..=n {
        for j in 0..=m {
            if i >= 1 && j >= 1 && s[i - 1] == t[j - 1] {
                dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1).min(dp[i - 1][j - 1]);
            } else if i >= 1 && j >= 1 {
                dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1).min(dp[i - 1][j - 1] + 1);
            } else if i >= 1 {
                dp[i][j] = dp[i - 1][j] + 1;
            } else if j >= 1 {
                dp[i][j] = dp[i][j - 1] + 1;
            }
        }
    }

    println!("{}", dp[n][m]);
}
