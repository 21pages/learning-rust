use protobuf::Message;
use protobuf_test::test;
fn main() {
    println!("test1");
    test_1();
    println!("test2");
    test_2();
    println!("test3");
    test_3();
}

fn test_1() {
    let mut msg1 = test::Msg1::new();
    msg1.set_a(1);
    msg1.b = 2;
    let v = msg1.write_to_bytes().unwrap();

    let msg2 = test::Msg2::parse_from_bytes(&v);
    println!("msg2:{:?}", msg2);

    let msg3 = test::Msg3::parse_from_bytes(&v);
    println!("msg3:{:?}", msg3);
}

fn test_2() {
    let mut msg2 = test::Msg2::new();
    msg2.set_a(1);
    msg2.b = 2;
    // msg2.set_c(3);
    // msg2.set_d(4);
    let v = msg2.write_to_bytes().unwrap();

    let msg1 = test::Msg1::parse_from_bytes(&v);
    println!("msg1:{:?}", msg1);

    let mut msg3 = test::Msg3::new();
    msg3.set_a(1);
    msg3.b = 2;
    // msg3.set_c(3);
    // msg3.set_d(4);
    let v = msg3.write_to_bytes().unwrap();

    let msg1 = test::Msg1::parse_from_bytes(&v);
    println!("msg1:{:?}", msg1);
}

fn test_3() {
    let mut msg2 = test::Msg2::new();
    msg2.set_a(1);
    msg2.b = 2;
    // msg2.set_c(3);
    // msg2.set_d(4);
    let v = msg2.write_to_bytes().unwrap();

    let msg2 = test::Msg2::parse_from_bytes(&v);
    println!("msg2:{:?}", msg2);
}
