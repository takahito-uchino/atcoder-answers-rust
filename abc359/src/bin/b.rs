use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 2],
    }

    let mut answer = 0;

    for i in 0..2 * n - 2 {
        if a[i] == a[i + 2] {
            answer += 1;
        }
    }

    println!("{}", answer)
}
