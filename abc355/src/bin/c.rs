use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut a: [usize; t],
    }

    for i in 0..t {
        a[i] -= 1;
    }

    let mut row = vec![0; n];
    let mut col = vec![0; n];
    let mut diag = vec![0; 2];

    for i in 0..t {
        let x = a[i] / n;
        let y = a[i] % n;

        row[x] += 1;
        if row[x] == n {
            println!("{}", i + 1);
            return;
        }

        col[y] += 1;
        if col[y] == n {
            println!("{}", i + 1);
            return;
        }

        if x == y {
            diag[0] += 1;
            if diag[0] == n {
                println!("{}", i + 1);
                return;
            }
        }

        if x + y == n - 1 {
            diag[1] += 1;
            if diag[1] == n {
                println!("{}", i + 1);
                return;
            }
        }
    }

    println!("-1");
}
