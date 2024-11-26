use mongodb::bson;
use serde::{Deserialize, Serialize};

// Define the YouTubeChannelRecord model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YouTubeChannelRecord {
    pub channel_name: String,
    pub channel_id: String,
    pub thumbnail: String,
}

// Define the VotePeriod model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VotePeriod {
    pub timestamp: i64,
    pub vote_count: i32,
}

// Define the PayCycleEvent model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PayCycleEvent {
    pub cycle_id: String,
    pub event_id: String,
    pub amount: f64,
    pub votes: i32,
    pub total_votes: i32,
    pub total_revenue: f64,
    pub devepub_loper_cut: f64,
    pub battler_cut: f64,
    pub transaction_cost: f64,
    pub event: String,
    pub paypal_transaction_id: String,
    pub timestamp: i64,
}

// Define the User model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub _id: bson::oid::ObjectId,
    pub username: String,
    pub email: String,
    pub temp_email: String,
    pub password: String,
    pub battler_score: f64,
    pub registration_timestamp: i64,
    pub invitation_code: String,
    pub account_status: String,
    pub youtube_channels: Vec<YouTubeChannelRecord>,
    pub youtube_thumbnail: String,
    pub youtube_channel_name: String,
    pub youtube_channel_id: String,
    pub youtube_state_code: String,
    pub twitter: String,
    pub twitter_name: String,
    pub twitter_token: String,
    pub twitter_token_secret: String,
    pub instagram_id: String,
    pub instagram_name: String,
    pub instagram_state_code: String,
    pub instagram_thumbnail: String,
    pub google_id: String,
    pub google_email: String,
    pub apple_id: String,
    pub apple_email: String,
    pub apple_state_code: String,
    pub apple_last_refresh: i64,
    pub paypal: String,
    pub highest_rank: i32,
    pub highest_rank_out_of: i32,
    pub votes_for: i32,
    pub votes_against: i32,
    pub judge_votes: i32,
    pub final_votes: i32,
    pub current_votes_for: i32,
    pub vote_periods: Vec<VotePeriod>,
    pub pay_cycle_events: Vec<PayCycleEvent>,
    pub password_reset_code: String,
    pub email_reset_code: String, // short string
    pub session_id: String,       // client cookie
    pub browser_id: String,       // client cookie
    pub device_id: String,        // client cookie
    // pub session_timestamp: i64,
    // pub account_type: String,
    pub matches_won: Vec<String>,       // match IDs aka W's
    pub matches_lost: Vec<String>,      // match IDs aka L's
    pub matches_withdrawn: Vec<String>, // match IDs aka takebacks
    pub work_a: Vec<String>,            // match IDs where the battler was calling out
    pub work_b: Vec<String>,            // match IDs where the battler was responding
    pub top_ten_callouts: Vec<String>,  // match IDs
    pub voter: i32,                     // there may be levels, but a value of 1 sigifies a voter
    pub battler: i32,                   // there may be levels, but a value of 1 sigifies a battler
    pub admin: i32,                     // 1 for admin,
    pub judge: i32,                     // 1 for judge, levels coming soon
    pub live_admin: i32,                // there may be levels, just 0 and 1 now
    pub one_hundred_badge: i32,         // for the early battlers (set as 1)
    pub first_tourney_badge: String, // for the first prestige tourney, including top eight and honorable mentions
}
