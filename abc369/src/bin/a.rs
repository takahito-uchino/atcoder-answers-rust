use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    if a == b {
        println!("1");
    } else if (a - b) % 2 == 0 {
        println!("3");
    } else {
        println!("2");
    }
}
