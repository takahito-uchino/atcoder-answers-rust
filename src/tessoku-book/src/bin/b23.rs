use num_traits::Pow;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    
    let mut x = Vec::new();
    let mut y = Vec::new();

    for (xi, yi) in xy {
        x.push(xi);
        y.push(yi);
    }

    let mut dp = vec![vec![f64::MAX; n]; 1 << n];
    dp[0][0] = 0.0;

    for i in 0..(1 << n) {
        for j in 0..n {
            if dp[i][j] < f64::MAX {
                for k in 0..n {
                    if i & (1 << k) == 0 {
                        let dist = (((x[j] - x[k]).pow(2) + (y[j] - y[k]).pow(2)) as f64).sqrt();
                        dp[i | (1 << k)][k] = f64::min(dp[i | (1 << k)][k], dp[i][j] + dist);
                    }
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1][0]);
}
