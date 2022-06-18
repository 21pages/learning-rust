use std::sync::Mutex;
use std::thread;
// use std::rc::Rc;
use std::sync::Arc;

struct T {
    a: i32,
    b: i32,
}

#[test]
fn test1() {
    let counter = Arc::new(Mutex::new(T { a: 0, b: 0 })); //mutex是共享内存, 不是保护代码段并发的
    let mut handles = vec![];

    let cnt = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for _ in 1..10 {
            let mut num = cnt.lock().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
            num.a += 1;
            println!("1");
        }
    });
    handles.push(handle);

    let cnt2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        let mut num = cnt2.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
        num.a += 1;
        println!("2");
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap()
    }
    println!("finish, result={:?}", counter.lock().unwrap().a);
}
