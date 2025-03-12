use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans: Vec<usize> = Vec::new();
    ans.extend(a[n - k..n].iter());
    ans.extend(a[0..n - k].iter());

    println!("{}", ans.into_iter().map(|num| num.to_string()).join(" "))
}
