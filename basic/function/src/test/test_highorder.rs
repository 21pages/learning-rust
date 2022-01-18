fn is_odd(n: i32) -> bool {
    n % 2 == 1
}

#[test]
fn test() {
    let sum: i32 = (0..=1000)
        .map(|n| n * n)
        .take_while(|&n| n < 500)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("{}", sum);
}
