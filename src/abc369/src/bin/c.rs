use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    if n < 2 {
        println!("{}", 2 * n - 1);
        return;
    }

    let mut ans = 2 * n - 1;

    for l in 0..n - 2 {
        let mut r = l + 1;
        let d = a[r] - a[l];

        while r + 1 < n && a[r + 1] - a[r] == d {
            r += 1;
        }

        let len = r - l + 1;
        if len >= 3 {
            ans += len - 2;
        }
    }

    println!("{}", ans)
}
