mod common;

use crate::common::setup;

#[tokio::test]
async fn health_check_returns_ok() {
    let server = setup().await;
    let response = server.get(&format!("{}/health", common::API)).await;
    response.assert_status_ok();
}
