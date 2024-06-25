use poise::serenity_prelude as serenity;
use dotenv::dotenv;
use commands::sons_of_forest::sof_start::sof_start;
use commands::sons_of_forest::sof_stop::sof_stop;
use commands::sons_of_forest::sof_restart::sof_restart;
use commands::ayah_random::ayah_random;
use commands::minecraft::minecraft_start::minecraft_start;
use commands::minecraft::minecraft_stop::minecraft_stop;
use commands::minecraft::minecraft_restart::minecraft_restart;

pub struct Data {}

mod commands;
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
                ayah_random(),
                minecraft_start(),
                minecraft_stop(),
                minecraft_restart()],
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
