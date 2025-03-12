use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut coordinates = xy.iter().map(|&(x, y)| (x, -y)).collect::<Vec<_>>();
    coordinates.sort_unstable();

    let sorted_y = coordinates.iter().map(|&(_, y)| -y).collect::<Vec<isize>>();

    let lis_length = get_lis_value(&sorted_y);
    println!("{}", lis_length);
}

fn get_lis_value(a: &[isize]) -> usize {
    let mut lis = vec![];

    for &item in a {
        match lis.binary_search(&item) {
            Ok(_) => continue,
            Err(pos) => {
                if pos == lis.len() {
                    lis.push(item);
                } else {
                    lis[pos] = item;
                }
            }
        }
    }
    lis.len()
}
