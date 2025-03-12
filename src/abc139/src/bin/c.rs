use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut answer = 0;
    let mut count = 0;

    for i in 0..n - 1 {
        if h[i] >= h[i + 1] {
            count += 1;
            answer = answer.max(count);
        } else {
            count = 0;
        }
    }

    println!("{}", answer)
}
