use std::time::Duration;

use subxtxt::connection::Connection;
use testcontainers::clients::Cli;

use crate::utils::TestContext;

mod utils;

#[tokio::test(flavor = "multi_thread")]
async fn connects_to_aleph_node() {
    let docker = Cli::default();
    let context = TestContext::new(&docker);

    let _connection = Connection::new(&context.node_address()).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn observes_progress() {
    let docker = Cli::default();
    let context = TestContext::new(&docker);

    let connection = Connection::new(&context.node_address()).await;

    let old_hash = connection.as_client().rpc().finalized_head().await.unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    let new_hash = connection.as_client().rpc().finalized_head().await.unwrap();

    assert_ne!(old_hash, new_hash);
}
