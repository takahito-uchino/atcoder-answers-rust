use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut rev = vec![0; n];

    for i in 0..n {
        rev[a[i] - 1] = i + 1;
    }

    for i in rev {
        print!("{} ", i);
    }
}
