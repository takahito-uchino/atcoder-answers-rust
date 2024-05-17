use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut red = 0;
    let mut blue = 0;

    for c in s.chars() {
        if c == '0' {
            red += 1;
        } else {
            blue += 1;
        }
    }

    println!("{}", red.min(blue) * 2)
}
