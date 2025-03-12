use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let mut dp = vec![vec![0usize; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = 0;
                if i >= 1 && c[i - 1][j] == '.' {
                    dp[i][j] += dp[i - 1][j];
                }
                if j >= 1 && c[i][j - 1] == '.' {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
