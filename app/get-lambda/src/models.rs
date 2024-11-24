use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReadModel {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_timestamp: DateTime<Utc>,
    pub updated_timestamp: DateTime<Utc>,
}

//"updated_at": "2024-11-24 00:21:50.328496803 UTC",
//    "name": "One",
//    "created_at": "2024-11-24 00:21:50.328496803 UTC",
//    "description": "Description",
//    "id": "2pH0Udu1CTCod7IrbJiguf4mBKK"
