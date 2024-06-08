use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut lowercases = 0;
    let mut uppercases = 0;

    for c in s.chars() {
        if c.is_lowercase() {
            lowercases += 1;
        } else if c.is_uppercase() {
            uppercases += 1;
        }
    }

    if lowercases < uppercases {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}
