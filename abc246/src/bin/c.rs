use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut a: [usize; n],
    }

    let mut answer = 0;
    for i in 0..n {
        answer += a[i];
    }

    let mut m = 0;
    for i in 0..n {
        m += a[i] / x;
    }

    m = m.min(k);
    answer -= m * x;
    k -= m;

    for i in 0..n {
        a[i] %= x;
    }

    a.sort_unstable();

    for i in (0..n).rev() {
        if k == 0 {
            break;
        }
        answer -= a[i];
        k -= 1;
    }

    println!("{}", answer)
}
