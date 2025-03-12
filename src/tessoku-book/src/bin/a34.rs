use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    }

    let mut grundy = vec![0; 100001];

    for i in 0..=100000 {
        let mut transit = vec![false, false, false];
        if i >= x {
            transit[grundy[i - x]] = true;
        }
        if i >= y {
            transit[grundy[i - y]] = true;
        }
        if !transit[0] {
            grundy[i] = 0;
        } else if !transit[1] {
            grundy[i] = 1;
        } else {
            grundy[i] = 2;
        }
    }

    let mut xor_sum = 0;

    for i in 0..n {
        xor_sum = xor_sum ^ grundy[a[i]];
    }

    println!("{}", if xor_sum != 0 {"First"} else {"Second"})
}
