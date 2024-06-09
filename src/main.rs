use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use commands::sof_start::sof_start;
use commands::sof_stop::sof_stop;
use commands::sof_restart::sof_restart;
use commands::ayah_random::ayah_random;

pub struct Data {}

mod commands {
    pub mod utils;
    pub mod sof_start;
    pub mod sof_stop;
    pub mod sof_restart;
    pub mod ayah_random;
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                sof_start(),
                sof_stop(),
                sof_restart(),
                ayah_random()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
