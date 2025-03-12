use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut min = usize::MAX;
    let mut answer = 0;

    for i in 0..n {
        min = min.min(p[i]);
        if min == p[i] {
            answer += 1;
        }
    }

    println!("{}", answer)
}
