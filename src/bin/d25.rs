type Number = u64;

const MODULUS: Number = 20201227;
const CARD_PUBLIC: Number = 9033205;
const DOOR_PUBLIC: Number = 9281649;
const START_NUMBER: Number = 7;

fn find_loop(subject: Number, target: Number) -> usize {
    let mut loop_count = 0;
    let mut val = START_NUMBER;
    while val != target {
        val = (val * subject) % MODULUS;
        loop_count += 1;
    }
    loop_count
}

fn do_loop(subject: Number, loops: usize) -> Number {
    let mut val = subject;
    for _ in 0..loops {
        val = (val * subject) % MODULUS
    }
    val
}

fn part1() {
    let door_loop = find_loop(START_NUMBER, DOOR_PUBLIC);
    let card_loop = find_loop(START_NUMBER, CARD_PUBLIC);
    let encryption_key = do_loop(DOOR_PUBLIC, card_loop);
    assert_eq!(encryption_key, do_loop(CARD_PUBLIC, door_loop));
    println!("{}", encryption_key);
}

fn main() {
    part1();
}
