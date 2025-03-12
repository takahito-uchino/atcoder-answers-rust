use proconio::input;

fn main() {
    input! {
        y: usize,
    }

    let mut answer = 366;

    if y % 100 == 0 && y % 400 != 0 {
        answer = 365;
    } else if y % 4 != 0 {
        answer = 365;
    }

    println!("{}", answer)
}
