use rocket_contrib::databases::mongodb;

// Define the PayCycle model
#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    reporter_username: String,
    report_status: i64,
}

// Define the MongoDB collection
mongodb!(Report, "report");
