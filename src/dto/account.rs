#[serde_with::serde_as]
#[derive(serde::Deserialize,  serde::Serialize, Clone)]
pub struct Account {
    pub login: String,
    pub password: String,
    pub site: String,
    pub url: Option<String>,
}