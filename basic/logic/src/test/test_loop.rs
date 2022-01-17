#[test]
fn test_loop() {
    let mut i = 0;
    loop {
        i += 1;
        if i == 3 {
            continue;
        }
        if i == 5 {
            break;
        }
        println!("{}", i);
    }
}

#[test]
fn test_label() {
    let mut i = 0;
    'outer: loop {
        'inter: loop {
            i += 1;
            if i == 3 {
                break 'inter;
            } else if i == 4 {
                break 'outer;
            }
            println!("inner {:?}", i);
        }
        println!("outer {:?}", i);
    }
}

#[test]
fn test_return_from_break() {
    let mut i = 0;
    let j = loop {
        i += 1;
        if i == 2 {
            break i;
        }
    };
    println!("{}", j); //2
}
