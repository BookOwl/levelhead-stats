use std::collections::{HashMap, HashSet};
use reqwest;
use serde_json;
use reqwest::header::{HeaderName, HeaderValue};

pub struct Level {
    pub avatar_id: String,
    pub created_at: String,
    pub creator_time: f64,
    pub cv: u64,
    pub is_daily_build: bool,
    pub game_version: String,
    pub level_id: String,
    pub locale: String,
    pub locale_id: u64,
    pub required_players: u64,
    pub stats: Stats,
    pub high_scores: Vec<Record>,
    pub fastest_times: Vec<Record>,
    pub tags: Vec<String>,
    pub tag_names: Vec<String>,
    pub title: String,
    pub in_tower: bool,
    pub in_tower_trial: bool,
    pub updated_at: String,
    pub user_id: String,
    pub user_alias: String,
    pub internal_id: String,
}

pub struct Stats {
    pub attempts: u64,
    pub successes: u64,
    pub clear_rate: f64,
    pub failure_rate: f64,
    pub diamonds: u64,
    pub likes: u64,
    pub favorites: u64,
    pub hidden_gem: u64,
    pub playtime: f64,
    pub players: u64,
    /// AKA Spice
    pub replay_value: f64,
    pub time_per_win: f64,
    pub exposure_bucks: u64,
}

pub enum RecordType {
    HighScore,
    FastestTime,
}

pub struct Record {
    pub record_type: RecordType,
    pub value: f64,
    pub user_id: String,
    pub user_alias: String,
    pub created_at: String,
}

pub struct Client {
    key: String,

}

impl Client {
    pub fn new(key: &str) -> Self {
        Client {
            key: key.to_string(),
        }
    }
    pub async fn levels_by_user(&self, user_id: &str) -> reqwest::Result<Vec<Level>> {
        let mut levels = vec![];
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(HeaderName::from_static("rumpus-delegation-key"), HeaderValue::from_str(&self.key).unwrap());
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
            .default_headers(headers)
            .gzip(true)
            .build()?;
        let mut tiebreak_id: Option<String> = None;
        let mut max_created_at: Option<String> = None;
        loop {
            let mut query = vec![("sort", "createdAt"),
                             ("limit", "32"),
                             ("userIds", &user_id),
                             ("includeStats", "true"),
                             ("includeRecords", "true"),
            ];
            if let (Some(id), Some(time)) = (tiebreak_id.as_ref(), max_created_at.as_ref()) {
                query.push(("tiebreakerItemId", &id));
                query.push(("maxCreatedAt", &time))
            }
            let response = client.get("https://www.bscotch.net/api/levelhead/levels")
                .query(&query)
                .send()
                .await?;
            let json: serde_json::Value = serde_json::from_str(&response.text().await?).expect("Rumpus should return valid JSON");
            let level_data = json["data"].as_array().expect("Rumpus should return an array");
            if level_data.len() == 0 {
                break
            }
            for level_obj in level_data.iter() {
                let stats = &level_obj["stats"];
                let stats = Stats {
                    attempts: stats["Attempts"].as_u64().unwrap(),
                    successes: stats["Successes"].as_u64().unwrap(),
                    clear_rate: stats["ClearRate"].as_f64().unwrap(),
                    failure_rate: stats["FailureRate"].as_f64().unwrap(),
                    diamonds: stats["Diamonds"].as_u64().unwrap(),
                    likes: stats["Likes"].as_u64().unwrap(),
                    favorites: stats["Favorites"].as_u64().unwrap(),
                    hidden_gem: stats["HiddenGem"].as_u64().unwrap(),
                    playtime: stats["PlayTime"].as_f64().unwrap(),
                    players: stats["Players"].as_u64().unwrap(),
                    replay_value: stats["ReplayValue"].as_f64().unwrap(),
                    time_per_win: stats["TimePerWin"].as_f64().unwrap(),
                    exposure_bucks: stats["ExposureBucks"].as_u64().unwrap(),
                };
                let high_scores = &level_obj["records"]["HighScore"].as_array().unwrap();
                let high_scores = high_scores.iter().map(|high_score| Record {
                    record_type: RecordType::HighScore,
                    value: high_score["value"].as_f64().unwrap(),
                    created_at: high_score["createdAt"].as_str().unwrap().to_string(),
                    user_id: high_score["userId"].as_str().unwrap().to_string(),
                    user_alias: String::new(),
                }).collect::<Vec<_>>();
                let fastest = &level_obj["records"]["FastestTime"].as_array().unwrap();
                let fastest = fastest.iter().map(|f| Record {
                    record_type: RecordType::FastestTime,
                    value: f["value"].as_f64().unwrap(),
                    created_at: f["createdAt"].as_str().unwrap().to_string(),
                    user_id: f["userId"].as_str().unwrap().to_string(),
                    user_alias: String::new(),
                }).collect::<Vec<_>>();
                let level = Level {
                    avatar_id: level_obj["avatarId"].as_str().unwrap().to_string(),
                    created_at: level_obj["createdAt"].as_str().unwrap().to_string(),
                    creator_time: level_obj["creatorTime"].as_f64().unwrap(),
                    cv: level_obj["cv"].as_u64().unwrap(),
                    is_daily_build: level_obj["dailyBuild"].as_bool().unwrap(),
                    game_version: level_obj["gameVersion"].as_str().unwrap().to_string(),
                    level_id: level_obj["levelId"].as_str().unwrap().to_string(),
                    locale: level_obj["locale"].as_str().unwrap().to_string(),
                    locale_id: level_obj["localeId"].as_u64().unwrap(),
                    required_players: level_obj["requiredPlayers"].as_u64().unwrap(),
                    stats: stats,
                    high_scores: high_scores,
                    fastest_times: fastest,
                    tags: level_obj["tags"].as_array().unwrap().iter().map(|d| (&d).as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    tag_names: level_obj["tagNames"].as_array().unwrap().iter().map(|d| (&d).as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    title: level_obj["title"].as_str().unwrap().to_string(),
                    in_tower: level_obj["tower"].as_bool().unwrap(),
                    in_tower_trial: level_obj["towerTrial"].as_bool().unwrap(),
                    updated_at: level_obj["updatedAt"].as_str().unwrap().to_string(),
                    user_id: level_obj["userId"].as_str().unwrap().to_string(),
                    user_alias: String::new(),
                    internal_id: level_obj["_id"].as_str().unwrap().to_string(),
                };
                levels.push(level);
            }
            tiebreak_id = Some(levels[levels.len() - 1].internal_id.clone());
            max_created_at = Some(levels[levels.len() - 1].created_at.clone());
            //println!("JSON: {:#?}", &json);
        }
        let mut user_ids = Vec::with_capacity(levels.len() * 7);
        for level in &levels {
            user_ids.push(level.user_id.clone());
            for record in level.high_scores.iter().chain(level.fastest_times.iter()) {
                user_ids.push(record.user_id.clone());
            }
        }
        let aliases = self.user_ids_to_aliases(&user_ids).await?;
        for level in levels.iter_mut() {
            level.user_alias = aliases.get(&level.user_id).unwrap().clone();
            for record in level.high_scores.iter_mut() {
                record.user_alias = aliases.get(&record.user_id).unwrap().clone();
            }
            for record in level.fastest_times.iter_mut() {
                record.user_alias = aliases.get(&record.user_id).unwrap().clone();
            }
        }
        Ok(levels)
    }
    pub async fn user_ids_to_aliases(&self, user_ids: &[String]) -> reqwest::Result<HashMap<String, String>> {
        let ids = user_ids.iter().collect::<HashSet<_>>();
        let ids = ids.iter().collect::<Vec<_>>();
        let mut aliases: HashMap<String, String> = HashMap::with_capacity(ids.len());
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(HeaderName::from_static("rumpus-delegation-key"), HeaderValue::from_str(&self.key).unwrap());
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
            .default_headers(headers)
            .gzip(true)
            .build()?;
        for chunk in ids.chunks(64) {
            let query = [("userIds", chunk.iter().map(|&&s| s.clone()).collect::<Vec<String>>().join(","))];
            let response = client.get("https://www.bscotch.net/api/levelhead/aliases")
                .query(&query)
                .send()
                .await?;
            let json: serde_json::Value = serde_json::from_str(&response.text().await?)
                .expect("Rumpus should return valid JSON");
            json["data"]
                .as_array().unwrap()
                .iter()
                .map(|d| (d["userId"].as_str().unwrap(), d["alias"].as_str().unwrap()))
                .for_each(|(id, alias)| {
                    aliases.insert(id.to_string(), alias.to_string());
                })
        }
        Ok(aliases)
    }
}