use tokio::task::JoinHandle;

async fn do_stuff_async() -> Result<(), &'static str> {
    Err("hello")
}

async fn more_async_work() -> Result<(), &'static str> {
    Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(err) => Err("handling failed"),
    }
}

#[tokio::main]
async fn main() {
    let handle1 = tokio::spawn(do_stuff_async());
    let handle2 = tokio::spawn(more_async_work());
    match tokio::try_join!(flatten(handle1), flatten(handle2)) {
        Ok(_val) => {
            // do something with the values
        }
        Err(err) => {
            println!("Failed with {}.", err);
        }
    }
}

/*
Failed with hello.
*/
