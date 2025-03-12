use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut sum = vec![0; 1000002];

    for (a, b) in ab {
        sum[a] += 1;
        sum[b + 1] -= 1;
    }

    for i in 0..1000001 {
        sum[i + 1] += sum[i];
    }

    println!("{}", sum.iter().max().unwrap());
}
