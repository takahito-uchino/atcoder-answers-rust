use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    let a_sum = a.iter().sum::<usize>();
    let mut answer = (x / a_sum) * n;
    let mut b_sum = (x / a_sum) * a_sum;

    for &val in &a {
        b_sum += val;
        answer += 1;
        if b_sum > x {
            println!("{}", answer);
            return;
        }
    }
}
