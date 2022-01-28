#[tokio::main]
async fn main() {
    let mut count = 0u8;

    loop {
        tokio::select! {
            // If you run this example without `biased;`, the polling order is
            // pseudo-random, and the assertions on the value of count will
            // (probably) fail.
            biased; //依次判断

            _ = async {}, if count < 1 => {
                count += 1;
                assert_eq!(count, 1);
            }
            _ = async {}, if count < 2 => {
                count += 1;
                assert_eq!(count, 2);
            }
            _ = async {}, if count < 3 => {
                count += 1;
                assert_eq!(count, 3);
            }
            _ = async {}, if count < 4 => {
                count += 1;
                assert_eq!(count, 4);
            }

            else => {
                break;
            }
        };
    }
}
