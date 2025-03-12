use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sorted_a = a.clone();
    sorted_a.sort_unstable();
    sorted_a.reverse();

    let min = sorted_a[1];

    println!("{}", a.iter().position(|&x| x == min).unwrap() + 1)
}
