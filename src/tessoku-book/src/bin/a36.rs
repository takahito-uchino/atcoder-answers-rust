use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k >= n * 2 - 2 && k % 2 == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
