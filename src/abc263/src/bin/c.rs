use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for comb in (1..=m).combinations(n) {
        println!("{}", comb.iter().map(|&num| num.to_string()).join(" "))
    }
}
