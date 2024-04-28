use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    let mut b = Vec::new();

    for _ in 0..n {
        input! {
            ai: isize,
            bi: isize,
        }
        a.push(ai);
        b.push(bi);
    }

    let answer1 = solve(1, 1, n, &a, &b);
    let answer2 = solve(1, 2, n, &a, &b);
    let answer3 = solve(2, 1, n, &a, &b);
    let answer4 = solve(2, 2, n, &a, &b);

    println!("{}", answer1.max(answer2).max(answer3).max(answer4))
}

fn solve(omote: isize, ura: isize, n: usize, a: &Vec<isize>, b: &Vec<isize>) -> isize {
    let mut sum = 0;

    for i in 0..n {
        let mut card1 = a[i];
        if omote == 2 {
            card1 = -a[i];
        }
        let mut card2 = b[i];
        if ura == 2 {
            card2 = -b[i]
        }
        if card1 + card2 >= 0 {
            sum += card1 + card2;
        }
    }

    sum
}
