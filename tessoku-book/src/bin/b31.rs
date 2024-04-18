use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", (n / 3) + (n / 5) + (n / 7) - (n / 15) - (n / 35) - (n / 21) + (n / 105))
}
