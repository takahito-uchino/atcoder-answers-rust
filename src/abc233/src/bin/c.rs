use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }

    let mut l = Vec::new();
    let mut a = Vec::new();

    for _ in 0..n {
        input! {
            _l: usize,
            _a: [usize; _l],
        }
        l.push(_l);
        a.push(_a);
    }
    let mut ans = 0;
    dfs(0, 1, n, x, &mut ans, &a);
    println!("{}", ans)
}

fn dfs(pos: usize, pro: usize, n: usize, x: usize, ans: &mut usize, a: &Vec<Vec<usize>>) {
    if pos == n {
        if pro == x {
            *ans += 1;
        }
        return;
    }
    for &c in &a[pos] {
        if pro > x / c {
            continue;
        }
        dfs(pos + 1, pro * c, n, x, ans, a);
    }
}
