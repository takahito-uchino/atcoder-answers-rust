use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [u64; n],
    }

    c.sort_unstable();

    let mut answer = 1;

    for i in 0..n {
        answer = answer * 0.max(c[i] - i as u64) % 1000000007;
    }

    println!("{}", answer)
}
