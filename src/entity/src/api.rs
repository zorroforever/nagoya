use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq,Default,Deserialize, Serialize)]
pub struct RegModel {
    pub account_name: Option<String>,
    pub password: Option<String>,
}