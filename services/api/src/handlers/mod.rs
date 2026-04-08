use axum::Json;
use serde_json::json;

pub async fn health() -> Json<serde_json::Value> {
    // TODO: Check database connectivity
    Json(json!({
        "status": "ok",
        "service": "api",
        "version": "0.1.0"
    }))
}

pub async fn get_transaction(
    axum::extract::Path(hash): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    // TODO: Query transaction from database
    Json(json!({
        "hash": hash,
        "data": null,
        "message": "Transaction lookup not yet implemented"
    }))
}

pub async fn get_address(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    // TODO: Query address from database
    Json(json!({
        "address": id,
        "data": null,
        "message": "Address lookup not yet implemented"
    }))
}
