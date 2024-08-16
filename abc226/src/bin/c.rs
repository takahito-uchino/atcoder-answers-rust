use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut t = Vec::new();
    let mut k = Vec::new();
    let mut a = Vec::new();

    for _ in 0..n {
        input! {
            _t: usize,
            _k: usize,
            _a: [usize; _k],
        }
        t.push(_t);
        k.push(_k);
        let mut e = Vec::new();
        for i in _a {
            e.push(i - 1);
        }
        a.push(e);
    }

    let mut used = vec![false; n];
    let mut answer = 0;

    used[n - 1] = true;

    for i in (0..n).rev() {
        if used[i] {
            answer += t[i];
            for j in 0..k[i] {
                used[a[i][j]] = true;
            }
        }
    }

    println!("{}", answer)
}
