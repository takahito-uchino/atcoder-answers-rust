use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    for i in 0..n {
        a[i] -= 1;
        b[i] -= 1;
        c[i] -= 1;
    }

    let mut count = vec![0; n];
    let mut answer = 0u64;

    for &i in &c {
        count[b[i]] += 1;
    }

    for &i in &a {
        answer += count[i] as u64;
    }

    println!("{}", answer)
}
