use reqwest;
use serde_json;
use reqwest::header::{HeaderName, HeaderValue};

#[derive(Debug)]
pub struct Level {
    avatar_id: String,
    created_at: String,
    creator_time: f64,
    cv: u64,
    is_daily_build: bool,
    game_version: String,
    level_id: String,
    locale: String,
    locale_id: u64,
    required_players: u64,
    stats: Stats,
    high_score: Vec<Record>,
    fastest_time: Vec<Record>,
    tags: Vec<String>,
    tag_names: Vec<String>,
    title: String,
    in_tower: bool,
    in_tower_trial: bool,
    updated_at: String,
    user_id: String,
    internal_id: String,
}

#[derive(Debug)]
pub struct Stats {
    attempts: u64,
    successes: u64,
    clear_rate: f64,
    failure_rate: f64,
    diamonds: u64,
    likes: u64,
    favorites: u64,
    hidden_gem: u64,
    playtime: f64,
    players: u64,
    /// AKA Spice
    replay_value: f64,
    time_per_win: f64,
    exposure_bucks: u64,
}

#[derive(Debug)]
pub enum RecordType {
    HighScore,
    FastestTime,
}

#[derive(Debug)]
pub struct Record {
    record_type: RecordType,
    value: f64,
    user_id: String,
    created_at: String,

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
        let mut client = reqwest::Client::builder()
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
            println!("Length of data: {}", level_data.len());
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
                }).collect::<Vec<_>>();
                let fastest = &level_obj["records"]["FastestTime"].as_array().unwrap();
                let fastest = fastest.iter().map(|f| Record {
                    record_type: RecordType::FastestTime,
                    value: f["value"].as_f64().unwrap(),
                    created_at: f["createdAt"].as_str().unwrap().to_string(),
                    user_id: f["userId"].as_str().unwrap().to_string(),
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
                    high_score: high_scores,
                    fastest_time: fastest,
                    tags: level_obj["tags"].as_array().unwrap().iter().map(|d| (&d).as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    tag_names: level_obj["tagNames"].as_array().unwrap().iter().map(|d| (&d).as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    title: level_obj["title"].as_str().unwrap().to_string(),
                    in_tower: level_obj["tower"].as_bool().unwrap(),
                    in_tower_trial: level_obj["towerTrial"].as_bool().unwrap(),
                    updated_at: level_obj["updatedAt"].as_str().unwrap().to_string(),
                    user_id: level_obj["userId"].as_str().unwrap().to_string(),
                    internal_id: level_obj["_id"].as_str().unwrap().to_string(),
                };
                println!("{}", &level.title);
                levels.push(level);
            }
            tiebreak_id = Some(levels[levels.len() - 1].internal_id.clone());
            max_created_at = Some(levels[levels.len() - 1].created_at.clone());
            //println!("JSON: {:#?}", &json);
        }
        Ok(levels)
    }
}