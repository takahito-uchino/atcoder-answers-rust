use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        ab: [(usize, String); n],
    }

    let mut answer = 0;

    for i in 0..n {
        if ab[i].1 == "E".to_string() {
            answer = answer.max(l - ab[i].0);
        } else {
            answer = answer.max(ab[i].0);
        }
    }

    println!("{}", answer)
}
