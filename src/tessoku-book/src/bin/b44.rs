use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut t = Vec::new();

    for i in 0..n {
        t.push(i);
    }

    for (i, x, y) in queries {
        if i == 1 {
            (t[x - 1], t[y - 1]) = (t[y - 1], t[x - 1]);
        } else {
            println!("{}", a[t[x - 1]][y - 1])
        }
    }
}
