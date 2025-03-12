use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n + 1],
        mut b: [usize; n],
    }

    let mut answer = 0;

    for i in 0..n {
        let left = a[i].min(b[i]);
        answer += left;
        a[i] -= left;
        b[i] -= left;

        let right = a[i + 1].min(b[i]);
        answer += right;
        a[i + 1] -= right;
        b[i] -= right;
    }

    println!("{}", answer)
}
