use rocket_contrib::databases::mongodb;

// Define the AdStat model
#[derive(Serialize, Deserialize, Debug)]
pub struct AdStat {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    short_id: String,
    a_camp: String,
    b_camp: String,
    event: String,
    timestamp: i64,
}

mongodb!(AdStats, "ad_stats");