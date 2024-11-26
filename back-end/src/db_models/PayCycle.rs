use rocket_contrib::databases::mongodb;

// Define the PayCycle model
#[derive(Serialize, Deserialize, Debug)]
pub struct PayCycle {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    cycle_id: String,
    total_votes: i64,
    total_revenue: f64,
    developer_cut: f64,
    battler_cut: f64,
    transaction_cost: f64,
    timestamp: i64,
    event: String,
}

// Define the MongoDB collection
mongodb!(PayCycle, "pay_cycles");
