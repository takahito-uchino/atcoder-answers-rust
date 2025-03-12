use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = vec![2000000000; n];
    dp[0] = 0;

    for i in 0..n - 2 {
        dp[i + 1] = dp[i + 1].min(dp[i] + a[i]);
        dp[i + 2] = dp[i + 2].min(dp[i] + b[i]);
    }
    dp[n - 1] = dp[n - 1].min(dp[n - 2] + a[n - 2]);

    println!("{}", dp[n - 1]);
}
