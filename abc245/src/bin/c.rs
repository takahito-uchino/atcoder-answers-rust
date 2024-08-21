use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];

    dp[0] = true;
    ep[0] = true;

    for i in 1..n {
        if dp[i - 1] {
            if a[i - 1].abs_diff(a[i]) <= k {
                dp[i] = true;
            }
            if a[i - 1].abs_diff(b[i]) <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] {
            if b[i - 1].abs_diff(a[i]) <= k {
                dp[i] = true;
            }
            if b[i - 1].abs_diff(b[i]) <= k {
                ep[i] = true;
            }
        }
    }

    println!("{}", if dp[n - 1] || ep[n - 1] { "Yes" } else { "No" })
}
