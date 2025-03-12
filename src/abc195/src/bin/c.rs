use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = 0;

    if n >= 1_000 {
        answer += n - 999;
    }

    if n >= 1_000_000 {
        answer += n - 999_999;
    }

    if n >= 1_000_000_000 {
        answer += n - 999_999_999;
    }

    if n >= 1_000_000_000_000 {
        answer += n - 999_999_999_999;
    }

    if n >= 1_000_000_000_000_000 {
        answer += n - 999_999_999_999_999;
    }

    println!("{}", answer)
}
