use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    let mut b = Vec::new();

    for _ in 0..n {
        input! {
            ai: usize,
            bi: usize,
        }
        a.push(ai);
        b.push(bi);
    }

    let mut max = 0;

    for i in 0..n {
        max = max.max(b[i] - a[i]);
    }

    println!("{}", a.iter().sum::<usize>() + max)
}
