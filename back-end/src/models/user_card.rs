use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSelect {
    pub _id: String,
    pub username: String,
    pub battler_score: f64,
    pub instagram_name: String,
    pub instagram_id: String,
    pub matches_won: i32,
    pub matches_lost: i32,
    pub matches_withdrawn: i32,
    pub callout: i32,
    pub response: i32,
    pub one_hundred_badge: i32,
    pub first_tourney_badge: String,
}
