use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                }
            }
            if j >= a[i - 1] {
                if dp[i - 1][j] || dp[i - 1][j - a[i - 1]] {
                    dp[i][j] = true;
                }
            }
        }
    }

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
