use proconio::input;

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }

    let mut answer = Vec::new();

    loop {
        if x == 1 && y == 1 {
            break;
        }
        answer.push((x, y));
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }

    answer.reverse();

    println!("{}", answer.len());
    for (l, r) in answer {
        println!("{} {}", l, r);
    }
}
