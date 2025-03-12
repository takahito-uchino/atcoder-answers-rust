use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if s == "ABC316" {
        println!("No");
        return;
    }
    let num: &usize = &s[3..6].parse().unwrap();
    if *num >= 1 && *num <= 349 {
        println!("Yes");
    } else {
        println!("No");
    }
}
