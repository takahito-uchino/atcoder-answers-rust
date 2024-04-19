use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        a: [usize; n],
    }

    let grundy = vec![0, 0, 1, 1, 2];

    let mut xor_sum = 0;

    for ai in a {
        xor_sum ^= grundy[ai % 5];
    }

    println!("{}", if xor_sum != 0 { "First" } else { "Second" })
}
