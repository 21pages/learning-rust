use protobuf::Message;
use protobuf_test::test;
fn main() {
    // println!("test1");
    // test_1();
    // println!("test2");
    // test_2();
    // println!("test3");
    // test_3();
    test_image_quality();
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

fn test_image_quality() {
    let mut msg_new = test::ImageQuality_Message_new::new();
    msg_new.quality = test::ImageQuality_new::Balanced.into();
    let v = msg_new.write_to_bytes().unwrap();
    let msg_old = test::ImageQuality_Message_old::parse_from_bytes(&v).unwrap();
    println!("msg_old: {:?}", msg_old.quality);

    let mut msg_old = test::ImageQuality_Message_old::new();
    msg_old.quality = test::ImageQuality_old::Balanced.into();
    let v = msg_new.write_to_bytes().unwrap();
    let msg_new = test::ImageQuality_Message_new::parse_from_bytes(&v).unwrap();
    println!("msg_new: {:?}", msg_new.quality);
}
