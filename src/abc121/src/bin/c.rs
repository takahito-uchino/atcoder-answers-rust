use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
    }

    let mut map = BTreeMap::new();

    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }

        let count = map.entry(a).or_insert(0);
        *count += b;
    }

    let mut answer = 0;

    for (k, v) in map {
        if m == v {
            answer += k * v;
            break;
        } else if m > v {
            m -= v;
            answer += k * v;
        } else {
            answer += k * m;
            break;
        }
    }

    println!("{}", answer)
}
