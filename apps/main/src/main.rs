
mod settings;
use settings::Settings;


fn main() {

    let settings = Settings::new().expect("Failed to load config");

    println!("{}", settings.telegram.api_id);
    println!("{}", settings.telegram.api_hash);
    println!("{}", settings.telegram.bot_token)
}
