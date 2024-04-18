use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut dp = vec![false; n + 1];
    dp[a] = true;

    for i in a..=n {
        if !dp[i - a] {
            dp[i] = true;
        } else if i >= b && !dp[i - b] {
            dp[i] = true;
        } else {
            dp[i] = false;
        }
    }

    println!("{}", if dp[n] { "First" } else { "Second" })
}
