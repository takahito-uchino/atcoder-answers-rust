use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    if t[2] == 'X' {
        let mut dp = vec![vec![0; 3]; s.len() + 1];
        for i in 0..=s.len() {
            for j in 0..=2 {
                if i >= 1 && j >= 1 && s[i - 1].to_uppercase().to_string().chars().next().unwrap() == t[j- 1] {
                    dp[i][j] = (dp[i - 1][j]).max(dp[i][j - 1]).max(dp[i - 1][j - 1] + 1);
                } else if i >= 1 && j >= 1 {
                    dp[i][j] = (dp[i - 1][j]).max(dp[i][j - 1]);
                } else if i >= 1 {
                    dp[i][j] = dp[i - 1][j];
                } else if j >= 1 {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
        if dp[s.len()][2] > 1 {
            println!("Yes");
            return;
        }
    }

    let mut dp = vec![vec![0; 4]; s.len() + 1];
    for i in 0..=s.len() {
        for j in 0..=3 {
            if i >= 1 && j >= 1 && s[i - 1].to_uppercase().to_string().chars().next().unwrap() == t[j - 1] {
                dp[i][j] = (dp[i - 1][j]).max(dp[i][j - 1]).max(dp[i - 1][j - 1] + 1);
            } else if i >= 1 && j >= 1 {
                dp[i][j] = (dp[i - 1][j]).max(dp[i][j - 1]);
            } else if i >= 1 {
                dp[i][j] = dp[i - 1][j];
            } else if j >= 1 {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    if dp[s.len()][3] > 2 {
        println!("Yes");
        return;
    }
    println!("No");
}
