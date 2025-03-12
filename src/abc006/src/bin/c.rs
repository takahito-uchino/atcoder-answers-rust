use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n * 2 > m || n * 4 < m {
        println!("-1 -1 -1");
        return;
    }

    let sum = m - 2 * n;

    println!("{} {} {}", n - (sum / 2) - (sum % 2), sum % 2, sum / 2);
}
