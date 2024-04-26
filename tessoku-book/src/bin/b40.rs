use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = vec![0; 100];

    for ai in a {
        count[ai % 100] += 1;
    }

    let mut answer = 0;

    for i in 1..50 {
        answer += count[i] * count[100 - i];
    }

    answer += count[0] * (count[0] - 1) / 2;
    answer += count[50] * (count[50] - 1) / 2;

    println!("{}", answer)
}
