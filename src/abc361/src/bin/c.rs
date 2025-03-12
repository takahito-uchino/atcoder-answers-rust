use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort_unstable();

    let mut min = usize::MAX;

    for i in 0..=k {
        min = min.min(a[i + n - k - 1] - a[i]);
    }

    println!("{}", min)
}
