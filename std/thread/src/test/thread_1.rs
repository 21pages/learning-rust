use std::thread;

#[test]
fn test_move() {
    let v = 0;
    let s = String::from("hello");

    thread::spawn(move || {
        println!("in thread:v:{}", v);
        println!("in thread:s:{}", s);
    });
    println!("out thread:v:{}", v); //i32并没有move进去, 因为是值
    // println!("out thread:s:{}", s); //string move进去了, 因为是引用
}

/* 用到任何外部变量, 都要用move */
// #[test]
// fn test_not_move() {
//     let v = 0;
//     let s = String::from("hello");

//     thread::spawn(/* move */ || { //如果不加move, ^^ may outlive borrowed value `v`, s可能在线程未执行时就drop了
//         println!("in thread:v:{}", v);
//         println!("in thread:s:{}", s);

//     });
//     println!("out thread:v:{}", v); 
//     println!("out thread:s:{}", s);
// }