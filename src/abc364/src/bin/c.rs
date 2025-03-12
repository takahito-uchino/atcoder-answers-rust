use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_unstable();
    a.reverse();
    b.sort_unstable();
    b.reverse();

    let mut answer = n;

    let mut a_sum = 0;
    for i in 0..n {
        a_sum += a[i];
        if a_sum > x {
            answer = answer.min(i + 1);
            break;
        }
    }

    let mut b_sum = 0;
    for i in 0..n {
        b_sum += b[i];
        if b_sum > y {
            answer = answer.min(i + 1);
            break;
        }
    }

    println!("{}", answer)
}
