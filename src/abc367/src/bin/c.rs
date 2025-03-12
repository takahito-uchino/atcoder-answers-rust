use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n],
    }

    let mut result = Vec::new();
    let mut current = Vec::new();

    generate_combinations(n, &mut current, &mut result);

    for v in result {
        let mut ok = true;
        for i in 0..n {
            if v[i] > r[i] {
                ok = false;
                break;
            }
        }
        let sum = v.iter().sum::<usize>();
        if sum % k != 0 {
            ok = false;
        }
        if ok {
            println!("{}", v.iter().map(|num| num.to_string()).join(" "))
        }
    }
}

fn generate_combinations(n: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
    if current.len() == n {
        result.push(current.clone());
        return;
    }
    for i in 1..=5 {
        current.push(i);
        generate_combinations(n, current, result);
        current.pop();
    }
}
