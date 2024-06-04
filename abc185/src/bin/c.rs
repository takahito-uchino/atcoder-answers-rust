use proconio::input;

fn main() {
    input! {
        l: usize,
    }

    let mut answer = l - 1;

    for i in 2..=11 {
        answer = answer * (l - i) / i;
    }

    println!("{}", answer)
}
