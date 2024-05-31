use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n],
    }

    let mut max = 0;

    for i in 0..n - 1 {
        max = max.max(a[i + 1] - a[i]);
    }

    max = max.max(k + a[0] - a[n - 1]);

    println!("{}", k - max)
}
