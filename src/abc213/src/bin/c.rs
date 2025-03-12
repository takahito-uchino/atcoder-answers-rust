use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut a = Vec::new();
    let mut b = Vec::new();
    for (_a, _b) in ab.into_iter() {
        a.push(_a);
        b.push(_b);
    }

    let a = compress(a);
    let b = compress(b);

    for i in 0..n {
        println!("{} {}", a[i] + 1, b[i] + 1)
    }
}

fn compress(v: Vec<usize>) -> Vec<usize> {
    let mut v = v
        .into_iter()
        .enumerate()
        .sorted_by_key(|val| val.1)
        .collect_vec();

    let mut now = v[0].1;
    let mut val = 0usize;
    for (_, x) in v.iter_mut() {
        if now != *x {
            now = *x;
            val += 1;
        }
        *x = val;
    }

    v.sort();
    v.into_iter().map(|(_, x)| x).collect_vec()
}
