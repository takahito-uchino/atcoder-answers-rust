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
    for (i, x) in a.iter().enumerate() {
        if i == *x {
            same += 1;
        }
    }

    let mut answer = same * (same - 1) / 2;
    for (i, j) in a.iter().enumerate() {
        if i < *j && a[*j] == i {
            answer += 1;
        }
    }

    println!("{}", answer)
}
