use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DiscordGuildDto {
    pub id: i32,
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub guild_id: i64,
    pub name: String,
    pub icon_hash: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DiscordGuildRoleDto {
    pub id: i32,
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub guild_id: i64,
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub role_id: i64,
    pub name: String,
    pub color: String,
    pub position: i16,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct DiscordGuildChannelDto {
    pub id: i32,
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub guild_id: i64,
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub channel_id: i64,
    pub name: String,
    pub position: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct PaginatedDiscordGuildRolesDto {
    pub roles: Vec<DiscordGuildRoleDto>,
    pub total: u64,
    pub page: u64,
    pub entries: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct PaginatedDiscordGuildChannelsDto {
    pub channels: Vec<DiscordGuildChannelDto>,
    pub total: u64,
    pub page: u64,
    pub entries: u64,
}

fn serialize_i64_as_string<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn deserialize_i64_from_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer)?
        .parse::<i64>()
        .map_err(D::Error::custom)
}
