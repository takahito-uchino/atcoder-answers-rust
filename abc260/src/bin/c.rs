use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut red = 1;
    let mut blue = 0;

    for _ in (2..=n).rev() {
        blue += red * x;
        red += blue;
        blue *= y;
    }

    println!("{}", blue)
}
