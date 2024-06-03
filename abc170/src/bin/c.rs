use proconio::input;

fn main() {
    input! {
        x: usize,
        n: usize,
        p: [usize; n],
    }

    if n == 0 {
        println!("{}", x);
        return;
    }

    let mut diff = usize::MAX;
    let mut answer = 0;

    for i in 0..=101 {
        if !p.contains(&i) && diff > x.abs_diff(i) {
            diff = x.abs_diff(i);
            answer = i;
        }
    }

    println!("{}", answer)
}
