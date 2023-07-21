use ptrkalm_hello_shuttle::hello;

#[tokio::test]
async fn hello_world_test_get() {
    let result = hello::hello_world().await;
    assert_eq!(result, "Hello, Shuttle!");
}

#[tokio::test]
async fn hello_world_2_test_get() {
    let result = hello::hello_world_2().await;
    assert_eq!(result, "Hello, Shuttle 2!");
}
