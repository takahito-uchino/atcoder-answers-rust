use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut ans = vec![0; n];
    let mut max = vec![0; n];

    max[n - 1] = h[n - 1];

    for i in (0..n - 1).rev() {
        max[i] = max[i + 1].max(h[i]);
    }

    println!("{}", ans.iter().map(|num| num.to_string()).join(" "));
}
