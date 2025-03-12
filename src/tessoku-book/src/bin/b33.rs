use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize, usize); n],
    }

    let mut xor_sum = 0;

    for i in 0..n {
        xor_sum = xor_sum ^ (ab[i].0 - 1);
        xor_sum = xor_sum ^ (ab[i].1 - 1);
    }

    println!("{}", if xor_sum != 0 { "First" } else { "Second" })
}
