use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut counts = vec![0; 101];

    for i in 0..n {
        counts[a[i]] += 1;
    }

    let mut answer = 0;

    for i in 1..=100 {
        if counts[i] >= 3 {
            answer += counts[i] * (counts[i] - 1) * (counts[i] - 2) / 6;
        }
    }

    println!("{}", answer);
}
