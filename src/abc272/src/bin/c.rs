use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut odds = Vec::new();
    let mut evens = Vec::new();

    for i in a {
        if i % 2 == 1 {
            odds.push(i);
        } else {
            evens.push(i);
        }
    }

    odds.sort_unstable();
    evens.sort_unstable();

    let mut max = 0;

    if odds.len() < 2 && evens.len() < 2 {
        println!("-1");
        return;
    } else if odds.len() < 2 {
        max = max.max(evens[evens.len() - 1] + evens[evens.len() - 2]);
    } else if evens.len() < 2 {
        max = max.max(odds[odds.len() - 1] + odds[odds.len() - 2]);
    } else {
        max = (evens[evens.len() - 1] + evens[evens.len() - 2])
            .max(odds[odds.len() - 1] + odds[odds.len() - 2]);
    }

    println!("{}", max);
}
