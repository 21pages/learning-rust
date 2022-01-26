async fn foo() -> Result<u8, String> {
    Ok(1)
}
async fn bar() -> Result<u8, String> {
    Ok(1)
}
#[test]
pub fn test() {
    let fut = async {
        foo().await?;
        bar().await?;
        //Ok (1)//^^ cannot infer type for type parameter `E` declared on the enum `Result`
        Ok::<(), String>(())
    };
    let output = futures::executor::block_on(fut);
    println!("{:?}", output); //Ok(())
}
