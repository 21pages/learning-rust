type NanoSecond = u64;
type Inch = u64;
type U64T = u64;

#[test]
fn test() {
    let a = 5 as NanoSecond;
    let b = 5 as Inch;
    let c = 5 as U64T;
    println!("{},{},{}", a, b, c);
}
