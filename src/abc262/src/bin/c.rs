use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 0..n {
        a[i] -= 1;
    }

    let mut same = 0;
    for i in 0..n {
        if i == a[i] {
            same += 1;
        }
    }

    let mut answer = same * (same - 1) / 2;
    for i in 0..n {
        if a[i] < n && i < a[i] && a[a[i]] == i {
            answer += 1;
        }
    }

    println!("{}", answer)
}
