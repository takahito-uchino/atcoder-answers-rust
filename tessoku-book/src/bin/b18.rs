use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                }
            }
            if j >= a[i - 1] {
                if dp[i - 1][j] || dp[i - 1][j - a[i - 1]] {
                    dp[i][j] = true;
                }
            }
        }
    }

    if !dp[n][s] {
        println!("-1");
        return;
    }

    let mut card = s;
    let mut answer = Vec::new();

    for i in (1..=n).rev() {
        if dp[i - 1][card] {
            card = card - 0;
        } else {
            card -= a[i - 1];
            answer.push(i);
        }
    }

    answer.reverse();

    println!("{}", answer.len());
    println!("{}", answer.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}
