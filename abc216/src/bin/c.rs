use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    let mut answer = Vec::new();

    while n > 0 {
        if n % 2 == 0 {
            answer.push('B');
            n /= 2;
        } else {
            answer.push('A');
            n -= 1;
        }
    }

    answer.reverse();
    println!("{}", answer.iter().collect::<String>())
}
