use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
    }

    let mut answer = Vec::new();

    for i in 1..l {
        answer.push(i);
    }

    for i in (l..=r).rev() {
        answer.push(i);
    }

    for i in r + 1..=n {
        answer.push(i);
    }

    println!("{}", answer.iter().map(|&num| num.to_string()).join(" "))
}
