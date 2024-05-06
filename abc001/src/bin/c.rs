use proconio::input;

fn main() {
    input! {
        deg: f64,
        dis: f64,
    }

    let dir = deg / 10.;
    let answer = if 11.25 <= dir && dir < 33.75 {
        "NNE"
    } else if 33.75 <= dir && dir < 56.25 {
        "NE"
    } else if 56.25 <= dir && dir < 78.75 {
        "ENE"
    } else if 78.75 <= dir && dir < 101.25 {
        "E"
    } else if 101.25 <= dir && dir < 123.75 {
        "ESE"
    } else if 123.75 <= dir && dir < 146.25 {
        "SE"
    } else if 146.25 <= dir && dir < 168.75 {
        "SSE"
    } else if 168.75 <= dir && dir < 191.25 {
        "S"
    } else if 191.25 <= dir && dir < 213.75 {
        "SSW"
    } else if 213.75 <= dir && dir < 236.25 {
        "SW"
    } else if 236.25 <= dir && dir < 258.75 {
        "WSW"
    } else if 258.75 <= dir && dir < 281.25 {
        "W"
    } else if 281.25 <= dir && dir < 303.75 {
        "WNW"
    } else if 303.75 <= dir && dir < 326.25 {
        "NW"
    } else if 326.25 <= dir && dir < 348.75 {
        "NNW"
    } else {
        "N"
    };

    let wind = (dis / 6.).round() / 10.;

    let w = if wind <= 0.2 {
        0
    } else if 0.3 <= wind && wind <= 1.5 {
        1
    } else if 1.6 <= wind && wind <= 3.3 {
        2
    } else if 3.4 <= wind && wind <= 5.4 {
        3
    } else if 5.5 <= wind && wind <= 7.9 {
        4
    } else if 8.0 <= wind && wind <= 10.7 {
        5
    } else if 10.8 <= wind && wind <= 13.8 {
        6
    } else if 13.9 <= wind && wind <= 17.1 {
        7
    } else if 17.2 <= wind && wind <= 20.7 {
        8
    } else if 20.8 <= wind && wind <= 24.4 {
        9
    } else if 24.5 <= wind && wind <= 28.4 {
        10
    } else if 28.5 <= wind && wind <= 32.6 {
        11
    } else {
        12
    };

    if w == 0 {
        println!("C 0");
        return;
    }

    println!("{} {}", answer, w);
}
