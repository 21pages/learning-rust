#[test]
fn test_old_loop() {
    let mut op = Some(0);
    loop {
        println!("{:?}", op);
        match op {
            Some(i) => {
                if i < 9 {
                    op = Some(i + 1);
                } else {
                    op = None;
                }
            }
            _ => {
                break;
            }
        }
    }
}

#[test]
fn test_whilelet() {
    let mut op = Some(0);
    while let Some(i) = op {
        println!("{:?}", op);
        if i < 9 {
            op = Some(i + 1);
        } else {
            op = None;
        }
    }
}
