use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; n]; m],
    }

    let mut dp = vec![vec![10i32.pow(9); 1 << n]; m + 1];
    dp[0][0] = 0;

    for i in 1..=m {
        for j in 0..(1 << n) {
            let mut already = vec![0; n];
            for k in 0..n {
                already[k] = if (j >> k) & 1 == 0 { 0 } else { 1 };
            }

            let mut v = 0;
            for k in 0..n {
                if already[k] == 1 || a[i - 1][k] == 1 {
                    v |= 1 << k;
                }
            }

            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            dp[i][v] = dp[i][v].min(dp[i - 1][j] + 1);
        }
    }

    let answer = dp[m][(1 << n) - 1];
    if answer == 10i32.pow(9) {
        println!("-1");
    } else {
        println!("{}", answer);
    }
}
