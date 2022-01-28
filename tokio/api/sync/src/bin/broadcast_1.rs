use tokio::sync::broadcast;

/* 
多对多:rx通过tx.subscribe()获得, tx的消息将被每个rx消费
*/

#[tokio::main]
async fn main() {
    let (tx , mut rx1) = broadcast::channel::<String>(16);
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), "hello".to_string());
        assert_eq!(rx1.recv().await.unwrap(), "world".to_string());
    });
    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), "hello".to_string());
        assert_eq!(rx2.recv().await.unwrap(), "world".to_string());
    });
    let s1 = String::from("hello");
    tx.send(s1).unwrap();
    // println!("s1:{}", s1); //moved
    tx.send("world".into()).unwrap();
}
