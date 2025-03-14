#![allow(unused)]

use anyhow::Result;


#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_get("/hello?name=Brian").await?.print().await;

    Ok(())
}

#[tokio::test]
async fn quick_dev2() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:3000")?;

    client.do_get("/hello/Brian").await?.print().await;

    Ok(())
}