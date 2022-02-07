use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Entiy {
    pub count: i32,
}

#[tokio::main]
async fn main() {
    let count = Arc::new(Mutex::new(Entiy { count: 0 }));
    let mut v = vec![];
    for i in 0..5 {
        println!("{} 1", i);
        let count_cloned = count.clone();
        let handle = tokio::spawn(async move {
            println!("{} 2", i);

            //方式一:卡
            // count_cloned.lock().await.count = count_cloned.lock().await.count + 1;

            //方式二:不卡
            let tmp = count_cloned.lock().await.count + 1; //语句结束锁即被释放, 要遇见分号;
            count_cloned.lock().await.count = tmp;

            println!("{} 3", i);
            println!(
                "task {} start, value:{:?}",
                i,
                count_cloned.lock().await.count
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("task {} end", i);
        });
        v.push(handle);
    }

    for h in v {
        let _ = h.await;
    }

    println!("finish.");
}

/*
0 1
1 1
0 2
0 3
task 0 start, value:1
2 1
1 2
1 3
task 1 start, value:2
3 1
2 2
2 3
task 2 start, value:3
4 1
3 2
3 3
task 3 start, value:4
4 2
4 3
task 4 start, value:5
task 4 end
task 1 end
task 0 end
task 2 end
task 3 end
finish.
*/
