use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }

    a.sort_unstable();

    for xi in x {
        let position = match a.binary_search_by(|&num| num.cmp(&xi)) {
            Ok(index) | Err(index) => index,
        };

        let count = n - position;
        println!("{}", count);
    }
}
