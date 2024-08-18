use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; x + 1]; n + 1];

    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=x {
            if dp[i][j] {
                if j + ab[i].0 <= x {
                    dp[i + 1][j + ab[i].0] = true;
                }
                if j + ab[i].1 <= x {
                    dp[i + 1][j + ab[i].1] = true;
                }
            }
        }
    }

    println!("{}", if dp[n][x] { "Yes" } else { "No" })
}
