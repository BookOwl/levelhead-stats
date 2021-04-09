mod client;

static KEY: &'static str = include_str!("../rumpus_key");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = client::Client::new(&KEY);
    let levels = client.levels_by_user("0qvgb6").await?;
    println!("{:#?}", levels[0]);
    Ok(())
}
