use testcontainers::{clients::Cli, Container, RunnableImage};

use crate::utils::aleph_image::AlephImage;

mod aleph_image;

/// Base URL of the node: we are running containers in the network host mode.
pub const BASE_URL: &str = "ws://127.0.0.1";

/// Dockerized testing environment.
pub struct TestContext<'container> {
    /// The running container with the Aleph chain.
    pub node_container: Container<'container, AlephImage>,
    /// Exposed WS port of the chain.
    pub node_port: u16,
}

impl<'container> TestContext<'container> {
    /// Creates new `TestContext` out of `docker` CLI.
    ///
    /// # Arguments
    ///
    /// * `docker`: Docker client API
    ///
    /// # Networking and ports
    ///
    /// We are running the containers in the host network. Therefore, in order to allow for parallel
    /// test execution, we are choosing random ports (and ignoring potential conflicts).
    ///
    /// # Rationale for `docker` as an argument
    ///
    /// We cannot create CLI here, since it is referenced by the container.
    pub fn new<'docker: 'container>(docker: &'docker Cli) -> Self {
        let (port, rpc_port, ws_port, validator_port) = rand::random();

        let image = RunnableImage::from(AlephImage::new(port, rpc_port, ws_port, validator_port))
            .with_network("host");
        let node_container = docker.run(image);

        Self {
            node_container,
            node_port: ws_port,
        }
    }

    /// Returns the web socket address of the node.
    pub fn node_address(&self) -> String {
        format!("{}:{}", BASE_URL, self.node_port)
    }
}
