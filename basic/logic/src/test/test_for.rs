#[test]
fn test_for1() {
    let mut s1 = String::from("");
    for i in 1..101 {
        s1 += &format!("{}", i)
    }
    let mut s2 = String::from("");
    for i in 1..=100 {
        s2.push_str(&format!("{}", i));
    }
    assert_eq!(s1, s2);
}

#[test]
fn test_for2() {
    let names = vec!["hello", "world"];
    for n in names.iter() {
        //引用迭代, names仍可用,borrowed
        match *n {
            "hello" => println!("hello"),
            _ => println!("rust"),
        }
    }
    println!("{:?}", names);

    for n in names.into_iter() {
        //取值迭代, names不可用,moved
        match n {
            "hello" => println!("hello"),
            _ => println!("rust"),
        }
    }
    //println!("{:?}", names);

    let mut names = vec!["hello", "world"];
    for n in names.iter_mut() {
        *n = match n {
            &mut "hello" => "HELLO",
            _ => "RUST",
        }
    }
    println!("{:?}", names);
}
