use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricEntry {
    pub tenant: String,
    pub unix: u64,
    pub endpoint: String,
    pub method: String,
    pub bytes: usize,
    pub ms: f64,
}

pub struct MetricsCollector {
    tenant: String,
}

impl MetricsCollector {
    pub fn new(tenant: String) -> Self {
        Self { tenant }
    }

    pub fn create_entry(
        &self,
        endpoint: String,
        method: String,
        bytes: usize,
        ms: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let unix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        let entry = MetricEntry {
            tenant: self.tenant.clone(),
            unix,
            endpoint,
            method,
            bytes,
            ms: ms as f64 / 1_000.0,
        };

        Ok(serde_json::to_string(&entry)?)
    }
}

// Constants for metrics
pub const METRICS_KEY: &str = "_metrics";
