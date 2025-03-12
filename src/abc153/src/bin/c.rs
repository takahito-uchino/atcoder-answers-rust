use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }

    if n <= k {
        println!("0");
        return;
    }

    h.sort_unstable();

    println!("{}", h[0..n - k].iter().sum::<usize>())
}
