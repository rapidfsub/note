use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    stdin().read_line(&mut buf).unwrap();
    let distances: Vec<i64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    buf.clear();

    stdin().read_line(&mut buf).unwrap();
    let prices: Vec<i64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    buf.clear();

    let mut min_price = prices[0];
    let mut distance_acc = distances[0];
    let mut result = 0;
    for index in 1..(n - 1) {
        let distance = distances[index];
        let price = prices[index];
        if price < min_price {
            result += min_price * distance_acc;
            min_price = price;
            distance_acc = 0;
        }
        distance_acc += distance;
    }
    result += min_price * distance_acc;
    println!("{result}");
}
