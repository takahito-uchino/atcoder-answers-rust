use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort_unstable();

    let len = d.len();
    let l = d[len / 2 - 1];
    let r = d[len / 2];

    println!("{}", r - l)
}
