use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [isize; n - 1],
    }

    let mut a = Vec::new();
    a.push(b[0]);

    for i in 0..n - 2 {
        a.push(b[i].min(b[i + 1]));
    }
    a.push(b[n - 2]);

    println!("{}", a.iter().sum::<isize>())
}
