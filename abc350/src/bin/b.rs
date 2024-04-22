use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q],
    }

    let mut teeth = vec![true; n];

    for ti in t {
        if teeth[ti - 1] {
            teeth[ti - 1] = false;
        } else {
            teeth[ti - 1] = true;
        }
    }

    let mut answer = 0;

    for b in teeth {
        if b {
            answer += 1;
        }
    }

    println!("{}", answer);
}
