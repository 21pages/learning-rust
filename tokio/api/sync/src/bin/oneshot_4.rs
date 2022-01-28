use tokio::sync::oneshot;

struct SenderDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}

impl Drop for SenderDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send("I got dropped");
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    let sender_drop = SenderDrop { sender: Some(tx) };
    drop(sender_drop);
    assert_eq!(rx.await, Ok("I got dropped"));
}
