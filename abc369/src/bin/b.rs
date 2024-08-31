use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    let mut s = Vec::new();

    for _ in 0..n {
        input! {
            _a: isize,
            _s: String,
        }
        a.push(_a);
        s.push(_s);
    }

    let mut ans = 0;
    let mut left = -1;
    let mut right = -1;

    for i in 0..n {
        if s[i] == "L" && left == -1 {
            left = a[i];
        } else if s[i] == "R" && right == -1 {
            right = a[i];
        } else if s[i] == "L" {
            ans += left.abs_diff(a[i]);
            left = a[i];
        } else if s[i] == "R" {
            ans += right.abs_diff(a[i]);
            right = a[i];
        }
    }

    println!("{}", ans)
}
