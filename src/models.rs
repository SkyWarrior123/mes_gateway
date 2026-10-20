use serde::{Deserialize, Serialize};

// Incoming raw payload from a third-party customer ERP/MES system 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawErpPayload {
    pub station_id: String,
    pub timestamp: String,
    pub part_code: Option<String>,
    pub status: String, // e.g., "OK", "FAIL", "ERR"
    pub metrics: serde_json::Value, // Unstructured or semi-structured custom fields
}

// standardized internal domain model
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct InternalTelemetry {
    pub machine_id: String,
    pub recorded_at: i64,
    pub item_sku: String,
    pub success: bool,
    pub cycle_time_ms: u64,
}