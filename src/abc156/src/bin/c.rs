use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut min = usize::MAX;

    for p in 1..=100 {
        let mut sum = 0;

        for i in &x {
            sum += (*i as isize - p as isize).pow(2);
        }

        min = min.min(sum as usize);
    }

    println!("{}", min)
}
