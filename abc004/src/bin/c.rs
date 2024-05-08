use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![1, 2, 3, 4, 5, 6];

    for i in 0..n % 30 {
        a.swap(i % 5, i % 5 + 1);
    }

    println!(
        "{}",
        a.iter().map(|&num| num.to_string()).collect::<String>()
    )
}
