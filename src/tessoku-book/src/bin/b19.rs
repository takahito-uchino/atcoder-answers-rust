use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![1_000_000_000_000_000; 100001]; n + 1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..100001 {
            if j < wv[i - 1].1 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = (dp[i - 1][j]).min(dp[i - 1][j - wv[i - 1].1] + wv[i - 1].0);
            }
        }
    }

    let mut answer = 0;
    for i in 0..100001 {
        if dp[n][i] <= w {
            answer = i;
        }
    }
    println!("{}", answer);
}
