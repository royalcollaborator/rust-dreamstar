use dioxus::prelude::*;

use crate::pages::{
    Badge, CallOut, ForgetPass, GoogleLogin, InstagramLogin, Invitation, Layout, LiveBattle,
    LiveBattleShow, Login, MainMenu, Match, PageNotFound, Profile, Response, Signup, AdminBattle, AdminUser, Policy
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    MainMenu,
    #[route("/login")]
    Login,
    #[route("/invitation")]
    Invitation,
    #[route("/signup")]
    Signup,
    #[route("/badge")]
    Badge,
    #[route("/profile")]
    Profile,
    #[route("/forget-password")]
    ForgetPass,
    #[route("/callout")]
    CallOut,
    #[route("/response")]
    Response,
    #[route("/live-battle-create")]
    LiveBattle,
    #[route("/policy")]
    Policy,
    #[route("/live-battle/:code")]
    LiveBattleShow { code: String },
    #[route("/match/:match_id")]
    Match { match_id: String },
    #[route("/auth/google/callback?:state&:code&:scope&:author&:prompt")]
    GoogleLogin {
        state: String,
        code: String,
        scope: String,
        author: String,
        prompt: String,
    },
    #[route("/auth/instagram/callback?:code")]
    InstagramLogin { code: String },
    // Admin
    #[route("/admin/user")]
    AdminUser,
    #[route("/admin/battle")]
    AdminBattle,
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
