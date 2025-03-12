use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 1..=n {
        for j in 0..=w {
            if j < wv[i - 1].0 {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = (dp[i - 1][j]).max(dp[i - 1][j - wv[i - 1].0] + wv[i - 1].1);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
