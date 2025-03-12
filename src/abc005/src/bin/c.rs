use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
    }

    let mut a_queue = VecDeque::from(a);
    let mut b_queue = VecDeque::from(b);
    let mut answer = true;

    while let Some(time) = b_queue.pop_front() {
        let mut is_sell = false;
        while let Some(take) = a_queue.pop_front() {
            if take > time {
                a_queue.push_front(take);
                break;
            } else if time <= take + t {
                is_sell = true;
                break;
            }
        }
        if !is_sell {
            answer = false;
            break;
        }
    }

    println!("{}", if answer { "yes" } else { "no" })
}
