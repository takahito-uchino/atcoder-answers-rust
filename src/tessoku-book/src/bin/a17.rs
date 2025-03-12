use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = Vec::new();
    dp.push(0);
    dp.push(a[0]);

    for i in 2..n {
        dp.push((dp[i - 1] + a[i - 1]).min(dp[i - 2] + b[i - 2]));
    }

    let mut place = n - 1;
    let mut answer = Vec::new();

    loop {
        answer.push(place + 1);
        if place == 0 {
            break;
        }
        if dp[place - 1] + a[place - 1] == dp[place] {
            place -= 1;
        } else {
            place -= 2;
        }
    }

    answer.reverse();

    println!("{}", answer.len());
    println!("{}", answer.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}
