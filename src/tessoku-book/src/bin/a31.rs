use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", (n / 3) + (n / 5) - (n / 15))
}
