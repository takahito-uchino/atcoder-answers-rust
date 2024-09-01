use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    let mut b = vec![Vec::new(); k];

    for i in 0..n {
        b[i % k].push(a[i]);
    }

    for i in 0..k {
        b[i].sort_unstable();
        b[i].reverse();
    }

    a.sort_unstable();

    let mut na = Vec::new();
    for i in 0..n {
        na.push(b[i % k].pop().unwrap());
    }

    println!("{}", if na == a { "Yes" } else { "No" })
}
