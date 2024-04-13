use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n - 1],
    }


    println!("{}", 0 - a.iter().sum::<isize>());
}
