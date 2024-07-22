use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [usize; n],
    }

    let mut b = Vec::new();

    for i in 0..n {
        b.push(a[i]);
        if i == k - 1 {
            b.push(x);
        }
    }

    println!("{}", b.iter().map(|&num| num.to_string()).join(" "))
}
