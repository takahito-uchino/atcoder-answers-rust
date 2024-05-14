use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrs: [(usize, usize, isize); n],
    }

    let mut imos = vec![0; m + 1];
    let mut total = 0;

    for (mut l, mut r, s) in lrs {
        l -= 1;
        r -= 1;
        imos[l] += s;
        imos[r + 1] -= s;
        total += s;
    }

    for i in 0..m {
        imos[i + 1] = imos[i] + imos[i + 1];
    }

    println!("{}", total - imos[0..m].iter().min().unwrap())
}
