use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    }

    let mut num = n - 2;
    while p[num] < p[num + 1] {
        num -= 1;
    }

    let mut m = n - 1;
    while p[num] < p[m] {
        m -= 1;
    }

    p.swap(num, m);

    let mut answer = Vec::new();
    answer.extend_from_slice(&p[..=num]);
    let mut tail = p[num + 1..].to_vec();
    tail.reverse();
    answer.extend(tail);

    println!("{}", answer.iter().map(|&num| num.to_string()).join(" "))
}
