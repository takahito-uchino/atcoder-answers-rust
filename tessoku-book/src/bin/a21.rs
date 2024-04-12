use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, usize); n],
    }

    let (mut p, mut a) = (vec![0; n + 2], vec![0; n + 2]);
    for (index, (pi, ai)) in pa.into_iter().enumerate() {
        p[index + 1] = pi;
        a[index + 1] = ai;
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for len in (0..=n - 2).rev() {
        for l in 1..=n - len {
            let r = l + len;

            let mut score1 = 0;
            if l <= p[l - 1] && p[l - 1] <= r {
                score1 = a[l - 1];
            }

            let mut score2 = 0;
            if l <= p[r + 1] && p[r + 1] <= r {
                score2 = a[r + 1];
            }

            if l == 1 {
                dp[l][r] = dp[l][r + 1] + score2;
            } else if r == n {
                dp[l][r] = dp[l - 1][r] + score1;
            } else {
                dp[l][r] = (dp[l - 1][r] + score1).max(dp[l][r + 1] + score2);
            }
        }
    }

    let mut answer = 0;

    for i in 1..=n {
        answer = answer.max(dp[i][i]);
    }

    println!("{}", answer);
}
