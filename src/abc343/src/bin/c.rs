use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = 1;

    for i in 1..1_000_001 {
        let v = i * i * i;
        if v > n {
            break;
        }
        if is_kaibun(v) {
            answer = v;
        }
    }
    println!("{}", answer)
}

fn is_kaibun(n: usize) -> bool {
    let str_n = n.to_string().chars().collect::<Vec<char>>();
    let mut str_n_rev = str_n.clone();
    str_n_rev.reverse();

    if str_n == str_n_rev {
        true
    } else {
        false
    }
}
