use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n - 1],
        b: [isize; n - 1],
    }

    let mut dp = vec![-1000000000; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[a[i] as usize - 1] = (dp[a[i] as usize - 1]).max(dp[i] + 100);
        dp[b[i] as usize - 1] = (dp[b[i] as usize - 1]).max(dp[i] + 150);
    }

    println!("{}", dp[n - 1]);
}
