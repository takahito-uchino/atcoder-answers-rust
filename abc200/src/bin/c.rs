use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = vec![0u64; 200];
    let mut answer = 0u64;

    for num in a {
        let r = num % 200;
        answer += count[r];
        count[r] += 1;
    }

    println!("{}", answer)
}
