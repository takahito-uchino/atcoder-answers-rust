use proconio::input;

fn main() {
    input! {
        mut x: usize,
        y: usize,
    }

    let mut answer = 0;

    while x <= y {
        answer += 1;
        x *= 2;
    }

    println!("{}", answer)
}
