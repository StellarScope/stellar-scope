use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// TODO: Shared type definitions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub hash: String,
    pub created_at: DateTime<Utc>,
    // TODO: Add more fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    // TODO: Add more fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: String,
    pub balance: String,
    // TODO: Add more fields
}
