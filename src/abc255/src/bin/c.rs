use std::isize;

use proconio::input;

fn main() {
    input! {
        x: isize,
        mut a: isize,
        mut d: isize,
        n: isize,
    }

    if d < 0 {
        let fi = a + d * (n - 1);
        a = fi;
        d *= -1;
    }

    let mut st = 0;
    let mut fi = n - 1;

    while st <= fi {
        let te = (st + fi) / 2;
        if (a + d * te) < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }

    let mut res = isize::MAX;

    for i in 0.max(st - 5)..=(n - 1).min(st + 5) {
        res = (a + d * i - x).abs().min(res);
    }

    println!("{}", res)
}
