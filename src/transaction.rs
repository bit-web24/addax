use chrono::{DateTime, Utc};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: DateTime<Utc>,
    pub signature: String,
    pub _id: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, signature: String) -> Self {
        let mut hasher = Sha256::new();
        let timestamp = Utc::now();

        let transaction_data = serde_json::json!({
            "sender": sender,
            "receiver": receiver,
            "amount": amount,
            "timestamp": timestamp,
            "signature": signature,
        });
        let transaction_data = serde_json::to_string(&transaction_data).unwrap();

        hasher.update(&transaction_data);
        let _id = hex::encode(hasher.finalize());

        Transaction {
            _id,
            sender,
            receiver,
            amount,
            timestamp,
            signature,
        }
    }
}
