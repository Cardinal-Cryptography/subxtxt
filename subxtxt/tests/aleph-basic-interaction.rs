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
