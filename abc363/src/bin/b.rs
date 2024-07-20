use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n],
    }

    l.sort_unstable();

    if l[n - p] > t {
        println!("0");
    } else {
        println!("{}", t - l[n - p]);
    }
}
