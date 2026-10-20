use crate::models::{InternalTelemetry, RawErpPayload};

#[derive(Debug)]
pub enum TransformationError {
    InvalidTimestamp,
    MissingSku,
    InvalidStatus,
}

/// Maps messy customer ERP data into our internal production format
pub fn map_erp_payload(raw: &RawErpPayload) -> Result<InternalTelemetry, TransformationError> {
    let recorded_at = raw.timestamp.parse::<i64>().map_err(|_| TransformationError::InvalidTimestamp)?;
    
    let item_sku = raw.part_code.clone().ok_or(TransformationError::MissingSku)?;
    
    let success = match raw.status.to_uppercase().as_str() {
        "OK" | "SUCCESS" | "PASS" => true,
        "FAIL" | "ERROR" | "ERR" => false,
        _ => return Err(TransformationError::InvalidStatus),
    };

    // Extract semi-structured metrics safely with fallbacks
    let cycle_time_ms = raw.metrics
        .get("cycle_time")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    Ok(InternalTelemetry {
        machine_id: raw.station_id.clone(),
        recorded_at,
        item_sku,
        success,
        cycle_time_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_successful_mapping() {
        let raw = RawErpPayload {
            station_id: "CNC-01".to_string(),
            timestamp: "1710000000".to_string(),
            part_code: Some("SKU-9988".to_string()),
            status: "OK".to_string(),
            metrics: json!({ "cycle_time": 1450 }),
        };

        let result = map_erp_payload(&raw).unwrap();
        assert_eq!(result.machine_id, "CNC-01");
        assert_eq!(result.item_sku, "SKU-9988");
        assert!(result.success);
        assert_eq!(result.cycle_time_ms, 1450);
    }
}