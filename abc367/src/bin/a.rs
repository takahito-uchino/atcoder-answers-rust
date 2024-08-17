use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if (a < b && c < a) || (a < b && b < c) || (b < c && c < a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
