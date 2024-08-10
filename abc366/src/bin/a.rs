use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: usize,
    }

    if t >= n / 2 + 1 || a >= n / 2 + 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
