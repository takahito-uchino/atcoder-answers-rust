use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut used = vec![false; 2 * n + 2];

    loop {
        for i in 1..=n * 2 + 1 {
            if !used[i] {
                println!("{}", i);
                used[i] = true;
                break;
            }
        }

        input! {
            result: usize,
        }

        if result == 0 {
            break;
        }

        used[result] = true;
    }
}
