use env_logger;
use BotOsint::{bot::start, jsonconfig::json};
#[tokio::main]
async fn main() {
    // iniciar o logger
    env_logger::init();
    let v = json::JsonBot::set_json(json::JsonBot::IdDonos);
    println!("aa {:?}", v);
    println!("Hello, world!");

    let result = start::start_bot().await;
    println!("{:?}", result);
}
