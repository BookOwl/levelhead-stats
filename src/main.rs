use std::error::Error;

use text_io::read;
use csv::Writer;

mod client;

static KEY: &'static str = include_str!("../rumpus_key");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Enter the levelhead user ID you want to download level statistics for:");
    let user_id: String = read!();
    let client = client::Client::new(&KEY);
    let levels = client.levels_by_user(&user_id).await?;
    let mut wrt = Writer::from_path(format!("level_data_{}.csv", user_id)).unwrap();
    wrt.write_record(&["name", "level_id", "created_at", "in_tower", "in_tower_trial", 
                        "tags", "high_score", "ribbon_holder", "shoe_time", "shoe_holder",
                        "required_players", "is_daily_build",
                        "attempts", "successes", "clear_rate", "failure_rate", "diamonds",
                        "likes", "favorites", "hidden_gem", "spice", "playtime", "players",
                        "time_per_win", "exposure_bucks"]).unwrap();
    for l in levels {
        wrt.write_record(&[
            l.title.clone(), l.level_id.clone(), l.created_at.clone(), format!("{:?}", l.in_tower), format!("{:?}", l.in_tower_trial), 
            l.tag_names.join(";"), format!("{:?}", l.high_scores[0].value), l.high_scores[0].user_alias.clone(), format!("{:?}", l.fastest_times[0].value), l.fastest_times[0].user_alias.clone(), 
            format!("{:?}", l.required_players), format!("{:?}", l.is_daily_build), 
            format!("{:?}", l.stats.attempts), format!("{:?}", l.stats.successes), format!("{:?}", l.stats.clear_rate), format!("{:?}", l.stats.failure_rate), format!("{:?}", l.stats.diamonds),
            format!("{:?}", l.stats.likes), format!("{:?}", l.stats.favorites), format!("{:?}", l.stats.hidden_gem), format!("{:?}", l.stats.replay_value), format!("{:?}", l.stats.playtime), format!("{:?}", l.stats.players),
            format!("{:?}", l.stats.time_per_win), format!("{:?}", l.stats.exposure_bucks)
        ]).unwrap();
    }
    wrt.flush();
    //println!("{:#?}", levels[levels.len() - 1]);
    Ok(())
}
