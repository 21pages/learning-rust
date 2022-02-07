use tokio::sync::broadcast;

/* 
多对多:rx通过tx.subscribe()获得, tx的消息将被每个rx消费
*/

#[tokio::main]
async fn main() {
    let (tx , _) = broadcast::channel::<String>(16);
    // drop(rx);
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    let mut task1 = tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), "hello".to_string());
        assert_eq!(rx1.recv().await.unwrap(), "world".to_string());
        println!("1 ok");
    });
    let mut task2 = tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), "hello".to_string());
        assert_eq!(rx2.recv().await.unwrap(), "world".to_string());
        println!("2 ok");
    });
    let s1 = String::from("hello");
    tx.send(s1).unwrap();
    // println!("s1:{}", s1); //moved
    tx.send("world".into()).unwrap();

    for _ in 0..2 {
        tokio::select!{
            _ = &mut task1 => {println!("task 1"); task1 = task1}, //重新赋值
            _ = &mut task2 => {println!("task 2"); task2 = task2},
        }
    }
}
