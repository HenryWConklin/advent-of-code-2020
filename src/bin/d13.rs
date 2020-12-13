use std::io::stdin;

fn ext_euclid(a: i128, b: i128) -> (i128, i128) {
    let d = a / b;
    let m = a % b;
    if m == 1 {
        (1, -d)
    } else {
        let (x, y) = ext_euclid(b, m);
        (y, x - (y * d))
    }
}

fn main() {
    let stdin = stdin();

    let mut start_time = String::new();
    stdin.read_line(&mut start_time).unwrap();
    let start_time = start_time.trim().parse::<i32>().expect(&start_time);

    let mut schedule = String::new();
    stdin.read_line(&mut schedule).unwrap();

    let res1 = schedule.trim().split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<i32>().expect(s))
        .map(|v| (v - (start_time % v), v))
        .min().unwrap();
    println!("{:?} {}", res1, res1.0 * res1.1);

    let res2 = schedule.trim().split(',')
        .enumerate()
        // filter ignored places
        .filter(|(_, x)| *x != "x")
        // parse
        .map(|(i, x)| (i, x.parse::<i128>().unwrap()))
        // to the equation t = -i mod x => t = x-i mod x so that thing stay unsigned
        .map(|(i, x)| (x - i as i128, x))
        .fold(
            (0, 1),
            |(a, p), (b, q)| {
                let (pi, qi) = ext_euclid(p, q);
                let pq = p * q;
                ((((a * ((q * qi) % pq)) % pq) + (b * ((p * pi) % pq) % pq)) % pq, pq)
            });
    println!("{:?}", res2);
}