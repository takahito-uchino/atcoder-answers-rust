use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let r#mod = 100000000;
    let mut a_mod = Vec::new();

    for ai in a {
        a_mod.push(ai % r#mod);
    }

    let mut answer = 0;

    for i in 0..n - 1 {
        for j in i + 1..n {
            answer += (a_mod[i] + a_mod[j]) % r#mod;
        }
    }

    println!("{}", answer)
}
