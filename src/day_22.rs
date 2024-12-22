fn mix(secret: &mut usize, val: usize) {
    *secret ^= val;
}

fn prune(secret: &mut usize) {
    *secret %= 16777216;
}

fn next_secret(mut n: usize) -> usize {
    let mul = n * 64;
    mix(&mut n, mul);
    prune(&mut n);
    let div = n / 32;
    mix(&mut n, div);
    prune(&mut n);
    let mul = n * 2048;
    mix(&mut n, mul);
    prune(&mut n);
    n
}

fn ones_digit(n: usize) -> usize {
    n - ((n / 10) * 10)
}

pub(crate) fn part_1(input: String) {
    let out = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .map(|mut s| {
            for i in 0..2000 {
                s = next_secret(s);
            }
            s
        })
        .reduce(|acc, e| acc + e)
        .unwrap();
    println!("Sum of secrets: {out}");
}

pub(crate) fn part_2(input: String) {
    todo!()
}

#[test]
fn test() {
    let mut secret = 123;
    let test_seq = [
        15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    ];
    let mut actual = vec![];
    for i in 0..10 {
        secret = next_secret(secret);
        actual.push(secret);
    }
    assert_eq!(test_seq.as_slice(), actual.as_slice());
}
