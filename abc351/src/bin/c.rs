use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut row = Vec::new();

    for i in 0..n {
        row.push(a[i]);
        loop {
            if row.len() >= 2 && row[row.len() - 1] == row[row.len() - 2] {
                let tmp = row[row.len() - 1];
                row.pop();
                row.pop();
                row.push(tmp + 1);
            } else {
                break;
            }
        }
    }

    println!("{}", row.len());
}
