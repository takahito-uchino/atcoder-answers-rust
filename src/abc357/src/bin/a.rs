use proconio::input;

fn main() {
    input! {
        n: usize,
        mut m: usize,
        h: [usize; n],
    }

    for i in 0..n {
        if m > h[i] {
            m -= h[i];
        } else if m == h[i] {
            println!("{}", i + 1);
            return;
        } else {
            println!("{}", i);
            return;
        }
    }

    println!("{}", n)
}
