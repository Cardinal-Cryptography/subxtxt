use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

const WARMUP_TIME_MILLIS: u64 = 10_000;

/// Wrapper around the latest `aleph-node` Docker image.
///
/// # Ready conditions
///
/// We consider container as healthy after `WARMUP_TIME_MILLIS` milliseconds. We do not wait for any
/// particular log. This is rather a brutal heuristic, but if the container is not ready right away,
/// then all tests should fail immediately.
#[derive(Clone, Debug)]
pub struct AlephImage {
    env_vars: HashMap<String, String>,
}

impl AlephImage {
    /// Creates image wrapper with custom ports.
    pub fn new(port: u16, rpc_port: u16, ws_port: u16, validator_port: u16) -> Self {
        Self {
            env_vars: HashMap::from([
                ("PORT".to_string(), port.to_string()),
                ("RPC_PORT".to_string(), rpc_port.to_string()),
                ("WS_PORT".to_string(), ws_port.to_string()),
                ("VALIDATOR_PORT".to_string(), validator_port.to_string()),
            ]),
        }
    }
}

impl Image for AlephImage {
    type Args = ();

    fn name(&self) -> String {
        "aleph-node".to_string()
    }

    fn tag(&self) -> String {
        "local-test".to_string()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::millis(WARMUP_TIME_MILLIS)]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
