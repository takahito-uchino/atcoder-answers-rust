use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: u64,
        mut ab: [(u64, u64); n],
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));

    for i in 0..n {
        if ab[i].0 > k {
            break;
        }
        k += ab[i].1;
    }

    println!("{}", k)
}
