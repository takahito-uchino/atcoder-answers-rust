use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sorted_a = a.clone();
    sorted_a.sort_unstable();
    let max = sorted_a[sorted_a.len() - 1];
    let sub_max = sorted_a[sorted_a.len() - 2];

    for i in a {
        if i == max {
            println!("{}", sub_max)
        } else {
            println!("{}", max)
        }
    }
}
