use proconio::input;

fn main() {
    input! {
        n: usize,
        ng1: usize,
        ng2: usize,
        ng3: usize,
    }

    if n == ng1 || n == ng2 || n == ng3 {
        println!("NO");
        return;
    }

    let mut dp = vec![101; 309];

    dp[n] = 0;
    for i in (0..n).rev() {
        if i == ng1 || i == ng2 || i == ng3 {
            continue;
        }
        let mut min_steps = usize::MAX;
        if i + 1 <= n {
            min_steps = min_steps.min(dp[i + 1]);
        }
        if i + 2 <= n {
            min_steps = min_steps.min(dp[i + 2]);
        }
        if i + 3 <= n {
            min_steps = min_steps.min(dp[i + 3]);
        }
        if min_steps < usize::MAX {
            dp[i] = min_steps + 1;
        }
    }

    println!("{}", if dp[0] <= 100 { "YES" } else { "NO" });
}

