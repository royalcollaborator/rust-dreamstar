use serde::{Deserialize, Serialize};

// Define the Vote model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub match_id: String,
    pub voter_name: String,
    pub voter_youtube_channel_name: String,
    pub voter_youtube_channel_id: String,
    pub voter_instagram_name: String,
    pub voter_twitter_name: String,
    pub voter_twitter: String,
    pub timestamp: i64,
    pub a_camp_votes: i32,
    pub b_camp_votes: i32,
    pub statement: String,
    pub vote_type: i32,
    pub thumbnail: String,
    pub bitcoin_transaction_id: String,
    pub satoshi_amount: i64,
    pub dollar_amount: f64,
    pub signature_img_file_id: String,
}
