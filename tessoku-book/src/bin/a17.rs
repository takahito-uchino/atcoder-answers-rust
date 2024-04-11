use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = Vec::new();
    dp.push(0);
    dp.push((h[0] - h[1]).abs());

    for i in 2..n {
        dp.push(((h[i] - h[i - 1]).abs() + dp[i - 1]).min((h[i] - h[i - 2]).abs() + dp[i - 2]));
    }

    let mut place = n - 1;
    let mut answer = Vec::new();

    loop {
        answer.push(place + 1);
        if place == 0 {
            break;
        }
        if dp[place - 1] + (h[place - 1] - h[place]).abs() == dp[place] {
            place -= 1;
        } else {
            place -= 2;
        }
    }

    answer.reverse();

    println!("{}", answer.len());
    println!("{}", answer.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}
