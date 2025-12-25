use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationParam {
    #[serde(default)]
    pub page: u64,
    #[serde(default = "default_entries")]
    pub entries: u64,
}

fn default_entries() -> u64 {
    10
}
