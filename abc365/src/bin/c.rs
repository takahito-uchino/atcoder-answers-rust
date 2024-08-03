use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }

    let total = a.iter().sum::<usize>();

    if total <= m {
        println!("infinite");
        return;
    }

    a.sort_unstable();
}

fn search(arr: Vec<usize>, m: usize) -> usize {
    let n = arr.len();
    let mut l = 0;
    let mut r = arr[n - 1];
    let mut x = (l + r) / 2;

    while l <= r {
        let total = get_total(arr, x);
        if total 
    }
}

fn get_total(arr: Vec<usize>, x: usize) -> usize {
    let mut total = 0;

    for i in arr {
        total += i.min(x);
    }

    total
}
