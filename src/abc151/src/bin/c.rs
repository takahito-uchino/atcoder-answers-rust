use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ps: [(usize, String); m],
    }

    let mut ac = vec![false; n];
    let mut wa = vec![0; n];

    for (p, s) in ps {
        if s == "AC" {
            ac[p - 1] = true;
        }
        if s == "WA" && !ac[p - 1] {
            wa[p - 1] += 1;
        }
    }

    let mut correct = 0;
    let mut penalty = 0;

    for i in 0..n {
        if ac[i] {
            correct += 1;
            penalty += wa[i];
        }
    }

    println!("{} {}", correct, penalty)
}
