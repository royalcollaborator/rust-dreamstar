pub mod auth;
pub mod battle;
pub mod layout;
pub mod page_not_found;
pub mod user;
pub mod admin;
pub mod policy;

pub use auth::forget_pass::ForgetPass;
pub use auth::google_login::GoogleLogin;
pub use auth::instagram_login::InstagramLogin;
pub use auth::invitation::Invitation;
pub use auth::login::Login;
pub use auth::signup::Signup;
pub use battle::callout::CallOut;
pub use battle::main_menu::MainMenu;
pub use battle::matchs::Match;
pub use battle::response::Response;
pub use layout::layout::Layout;
pub use page_not_found::PageNotFound;
pub use user::badge::Badge;
pub use user::profile::Profile;
pub use battle::live_battle::LiveBattle;
pub use battle::live_battle_show::LiveBattleShow;
pub use admin::battle::AdminBattle;
pub use admin::user::AdminUser;
pub use policy::Policy;
