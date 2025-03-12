use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut b = vec![n; n];
    let mut front = 0;

    for i in 0..n {
        if a[i] - 1 < 0 {
            front = i;
        } else {
            b[(a[i] - 1) as usize] = i;
        }
    }

    while front < n {
        print!("{} ", front + 1);
        front = b[front];
    }
    println!()
}
