use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut answer = usize::MAX;
    let mut x = 0;
    let mut y = 0;

    while x < n && y < m {
        answer = answer.min(a[x].abs_diff(b[y]));
        if a[x] > b[y] {
            y += 1;
        } else {
            x += 1;
        }
    }

    println!("{}", answer)
}
