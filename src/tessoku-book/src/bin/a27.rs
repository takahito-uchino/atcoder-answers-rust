use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    while a >= 1 && b >= 1 {
        if a >= b {
            a %= b;
        } else {
            b %= a;
        }
    }

    println!("{}", if a != 0 { a } else { b })
}
