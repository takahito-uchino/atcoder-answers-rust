use proconio::{input, marker::Chars};

fn main() {
    input! {
        str: Chars,
    }

    let a = str[0].to_digit(10).unwrap() as isize;
    let b = str[1].to_digit(10).unwrap() as isize;
    let c = str[2].to_digit(10).unwrap() as isize;
    let d = str[3].to_digit(10).unwrap() as isize;

    if a + b + c + d == 7 {
        println!("{}+{}+{}+{}=7", a, b, c, d)
    } else if a - b + c + d == 7 {
        println!("{}-{}+{}+{}=7", a, b, c, d)
    } else if a + b - c + d == 7 {
        println!("{}+{}-{}+{}=7", a, b, c, d)
    } else if a + b + c - d == 7 {
        println!("{}+{}+{}-{}=7", a, b, c, d)
    } else if a - b - c + d == 7 {
        println!("{}-{}-{}+{}=7", a, b, c, d)
    } else if a - b + c - d == 7 {
        println!("{}-{}+{}-{}=7", a, b, c, d)
    } else if a + b - c - d == 7 {
        println!("{}+{}-{}-{}=7", a, b, c, d)
    } else {
        println!("{}-{}-{}-{}=7", a, b, c, d)
    }
}
