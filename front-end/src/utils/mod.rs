use serde::{Deserialize, Serialize};

pub mod js_binding;
pub mod request;
pub mod storage;
pub mod time;
pub mod util;
pub mod validation;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrResModel {
    pub cause: String,
}
