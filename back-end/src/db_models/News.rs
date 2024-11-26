use rocket_contrib::databases::mongodb;

// Define the NewsStatement model
#[derive(Serialize, Deserialize, Debug)]
pub struct NewsStatement {
    username: String,
    timestamp: i64,
    instagram_name: String,
    youtube_channel_name: String,
    youtube_channel_id: String,
    twitter_name: String,
    twitter: String,
    statement: String,
}

// Define the News model
#[derive(Serialize, Deserialize, Debug)]
pub struct News {
    #[serde(rename = "_id")]
    id: bson::oid::ObjectId,
    article_id: String,
    timestamp: i64,
    title: String,
    article: String,
    statement_sequence: Vec<NewsStatement>,
}

// Define the MongoDB collection
mongodb!(News, "news");
