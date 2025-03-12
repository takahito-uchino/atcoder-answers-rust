use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let result = f(n);
    for line in result {
        println!("{}", line);
    }
}

fn f(n: usize) -> Vec<String> {
    if n == 0 {
        return vec![String::from("#")];
    }
    let sub = f(n - 1);
    let l = sub.len();
    let mut ret = vec![vec!['.'; 3 * l]; 3 * l];

    for i in 0..3 {
        for j in 0..3 {
            if i != 1 || j != 1 {
                for m in 0..l {
                    for n in 0..l {
                        ret[i * l + m][j * l + n] = sub[m].chars().nth(n).unwrap();
                    }
                }
            }
        }
    }

    ret.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}
