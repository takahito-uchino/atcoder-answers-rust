use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|&(a, _)| a);

    let mut sum: usize = ab.iter().map(|(_, b)| b).sum();

    if sum <= k {
        println!("1");
        return;
    }

    for i in 0..ab.len() {
        if sum <= k {
            println!("{}", ab[i - 1].0 + 1);
            return;
        }
        sum -= ab[i].1;
    }

    println!("{}", ab[ab.len() - 1].0 + 1);
}
