use proconio::input;

fn judge(a: char, b: char) -> i32 {
    match (a, b) {
        (x, y) if x == y => -1,
        ('G', 'P') => 1,
        ('C', 'G') => 1,
        ('P', 'C') => 1,
        _ => 0,
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; 2 * n],
    }

    let mut rank: Vec<(i32, usize)> = (0..2 * n).map(|i| (0, i)).collect();

    for j in 0..m {
        for i in 0..n {
            let player1 = rank[2 * i].1;
            let player2 = rank[2 * i + 1].1;
            let result = judge(
                s[player1].chars().nth(j).unwrap(),
                s[player2].chars().nth(j).unwrap(),
            );
            if result != -1 {
                rank[2 * i + result as usize].0 -= 1;
            }
        }
        rank.sort_by(|a, b| a.cmp(b).then_with(|| a.1.cmp(&b.1)));
    }

    for &(_, i) in &rank {
        println!("{}", i + 1);
    }
}

