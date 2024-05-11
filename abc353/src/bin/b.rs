use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut a_queue = VecDeque::from(a);

    let mut answer = 1;
    let mut current = k;

    loop {
        if current < a_queue[0] {
            answer += 1;
            current = k;
        } else {
            let front = a_queue.pop_front().unwrap();
            current -= front;
        }

        if a_queue.len() == 0 {
            break;
        }
    }

    println!("{}", answer)
}
