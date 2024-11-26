use mongodb::bson;
use serde::{Deserialize, Serialize};
// Define the EmailRecord model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InvitationCode {
    #[serde(rename = "_id")]
    pub _id: bson::oid::ObjectId,
    pub email: String,
    pub code: String,
    pub time_stamp: i64,
}

// Define the MongoDB collection
// mongodb!(EmailRecords, "email_records");
