use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        h: [usize; n],
        ab: [(usize, usize); m],
    }

    let mut is_peek = vec![true; n];
    let mut answer = 0;

    for (a, b) in ab {
        if h[a - 1] > h[b - 1] {
            is_peek[b - 1] = false;
        } else if h[a - 1] < h[b - 1] {
            is_peek[a - 1] = false;
        } else {
            is_peek[a - 1] = false;
            is_peek[b - 1] = false;
        }
    }

    for b in is_peek {
        if b {
            answer += 1;
        }
    }

    println!("{}", answer)
}
