use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by_key(|k| k.1);

    let mut current_time = 0;
    let mut answer = 0;

    for i in 0..n {
        if current_time <= lr[i].0 {
            current_time = lr[i].1;
            answer += 1;
        }
    }

    println!("{}", answer);
}
