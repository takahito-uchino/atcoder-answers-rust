use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut current = 0;
    for i in a {
        current = (current + i).max(0);
    }

    println!("{}", current)
}
