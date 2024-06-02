use num_integer::gcd;
use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut sum = 0;

    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                sum += gcd(a, gcd(b, c))
            }
        }
    }

    println!("{}", sum)
}
