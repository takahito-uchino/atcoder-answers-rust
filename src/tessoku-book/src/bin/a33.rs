use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut xor_sum = a[0];

    for i in 1..n {
        xor_sum = xor_sum ^ a[i];
    }

    println!("{}", if xor_sum != 0 { "First" } else { "Second" })
}
