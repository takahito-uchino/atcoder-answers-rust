use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k {
            for l in 1..=m {
                if j + l <= k {
                    dp[i + 1][j + l] = (dp[i + 1][j + l] + dp[i][j]) % MOD;
                }
            }
        }
    }

    let mut ans = 0;

    for i in 1..=k {
        ans = (ans + dp[n][i]) % MOD;
    }

    println!("{}", ans)
}
