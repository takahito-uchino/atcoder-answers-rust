use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut memo = t;

    for i in 0..n * 2 {
        memo[(i + 1) % n] = memo[(i + 1) % n].min(memo[i % n] + s[i % n]);
    }

    for answer in memo {
        println!("{}", answer)
    }
}
