use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();

    a.push(1);
    a.push(1);

    for i in 2..n {
        a.push((a[i - 1] + a[i - 2]) % 1000000007);
    }

    println!("{}", a[n - 1]);
}
