use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }

    let mut sorted_a = a.clone();
    sorted_a.sort();
    let mut dp = vec![false; n + 1];
    let a1 = sorted_a[0];

    for i in a1..=n {
        for ai in &sorted_a {
            if i >= *ai && !dp[i - ai] {
                dp[i] = true;
            }
        }
    }

    println!("{}", if dp[n] { "First" } else { "Second" })
}
