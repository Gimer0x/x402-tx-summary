/// x402scan / AgentCash discovery validates the **live** v2 `Payment-Required` JSON using the
/// Bazaar extension shape: `extensions.bazaar.schema.properties.input.properties.body` or
/// `queryParams`, plus `...output.properties.example`. The stock `x402-axum` challenge omits
/// this, so resources register but fail strict checks and do not list in the UI.
pub fn enrich_payment_required_for_x402scan(mut value: serde_json::Value) -> serde_json::Value {
    if value.get("x402Version").and_then(|v| v.as_u64()) != Some(2) {
        return value;
    }
    if bazaar_discovery_schema_present(&value) {
        return value;
    }

    let bazaar = serde_json::json!({
        "schema": {
            "type": "object",
            "properties": {
                "input": {
                    "type": "object",
                    "properties": {
                        "queryParams": {
                            "type": "object",
                            "required": ["network", "tx_hash"],
                            "properties": {
                                "network": {
                                    "type": "string",
                                    "description": "Numeric EVM chain id.",
                                    "enum": ["1", "8453", "84532"]
                                },
                                "tx_hash": {
                                    "type": "string",
                                    "description": "0x-prefixed 32-byte transaction hash.",
                                    "pattern": "^0x[a-fA-F0-9]{64}$"
                                }
                            }
                        }
                    }
                },
                "output": {
                    "type": "object",
                    "properties": {
                        "example": {
                            "type": "object",
                            "properties": {
                                "schema_version": { "type": "string" },
                                "tx_hash": { "type": "string" },
                                "data": { "type": "object" }
                            }
                        }
                    }
                }
            }
        }
    });

    let Some(obj) = value.as_object_mut() else {
        return value;
    };
    let ext = obj
        .entry("extensions")
        .or_insert_with(|| serde_json::json!({}));
    if let Some(ext_obj) = ext.as_object_mut() {
        ext_obj.insert("bazaar".to_string(), bazaar);
    }
    value
}

pub fn bazaar_discovery_schema_present(value: &serde_json::Value) -> bool {
    let Some(ext) = value.get("extensions") else {
        return false;
    };
    let Some(bazaar) = ext.get("bazaar") else {
        return false;
    };
    let Some(schema) = bazaar.get("schema") else {
        return false;
    };
    let Some(props) = schema.get("properties") else {
        return false;
    };
    let Some(input) = props.get("input") else {
        return false;
    };
    let Some(input_props) = input.get("properties") else {
        return false;
    };
    let has_input = input_props.get("body").is_some() || input_props.get("queryParams").is_some();
    let Some(output) = props.get("output") else {
        return false;
    };
    let Some(output_props) = output.get("properties") else {
        return false;
    };
    let has_example = output_props.get("example").is_some();
    has_input && has_example
}