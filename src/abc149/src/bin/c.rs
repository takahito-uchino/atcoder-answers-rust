use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut i = x;

    while !is_prime(i) {
        i += 1;
    }

    println!("{}", i)
}

fn is_prime(x: usize) -> bool {
    if x <= 1 {
        return false;
    }

    for i in 2..=x.sqrt() {
        if x % i == 0 {
            return false;
        }
    }

    true
}
