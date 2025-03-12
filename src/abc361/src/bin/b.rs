use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        g: usize,
        h: usize,
        i: usize,
        j: usize,
        k: usize,
        l: usize,
    }

    if is_common(a, d, g, j) && is_common(b, e, h, k) && is_common(c, f, i, l) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_common(l1: usize, r1: usize, l2: usize, r2: usize) -> bool {
    !(r1 <= l2 || r2 <= l1)
}
