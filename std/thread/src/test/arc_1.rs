//RefCell\Rc\Box

// use std::borrow::BorrowMut;
//RefCell<T>/Rc<T> 与Mutex<T>/Arc<T>
//1、Mutex<T>提供内部可变性，类似于RefCell
//2、RefCell<T>/Rc<T>是非线程安全的， Mutex<T>/Arc<T>是线程安全的
use std::sync::Mutex;
use std::thread;
// use std::rc::Rc;
use std::sync::Arc;

#[test]
fn test1() {
    let counter = Arc::new(Mutex::new(0)); //mutex是共享内存, 不是保护代码段并发的
    let mut handles = vec![];
    for _ in 0..10 {
        let cnt = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cnt.lock().unwrap(); //num: MutexGuard<i32>, 出作用域自动unlock
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap()
    }
    println!("finish, result={:?}", counter.lock().unwrap())
}

//用Rc
// #[test]
// fn test2() {
//    //Rc<T> 不是线程安全的
//    let counter = Rc::new(Mutex::new(0));
//    let mut handles = vec![];

//    for _ in 0..10 {
//        let cnt = Rc::clone(&counter);
//        let handle = thread::spawn(move || { // move :`Rc<Mutex<i32>>` cannot be sent between threads safely
//                                                 //without move :`Rc<Mutex<i32>>` cannot be shared between threads safely
//            let mut num = cnt.lock().unwrap();
//            *num += 1;
//        });

//        handles.push(handle);
//    }

//    for handle in handles {
//        handle.join().unwrap();
//    }

//    println!("resut = {}", *counter.lock().unwrap());
//    println!("Hello, world!");
// }

//不用Arc
// #[test]
// fn test3() {
//    let counter = Mutex::new(0);
//    let mut handles = vec![];

//    for _ in 0..10 {
//        let handle = thread::spawn( move || { //加不加move都是:value moved into closure here, in previous iteration of loop
//            let mut num = counter.lock().unwrap();
//            *num += 1;
//        });

//        handles.push(handle);
//    }

//    for handle in handles {
//        handle.join().unwrap();
//    }

//    println!("resut = {}", *counter.lock().unwrap());
//    println!("Hello, world!");
// }