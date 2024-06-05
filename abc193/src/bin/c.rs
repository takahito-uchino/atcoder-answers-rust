use std::collections::BTreeSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut set = BTreeSet::new();

    for a in 2..=n.sqrt() {
        let mut x = a * a;
        while x <= n {
            set.insert(x);
            x *= a;
        }
    }

    println!("{}", n - set.len())
}
