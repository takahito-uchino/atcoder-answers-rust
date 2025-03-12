use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    for i in 1..=1000000 {
        let str_i = i.to_string();
        let str = format!("{}{}", str_i, str_i);
        let num = str.parse::<usize>().unwrap();

        if num > n {
            println!("{}", i - 1);
            return;
        }
    }
}
