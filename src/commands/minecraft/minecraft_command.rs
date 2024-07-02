use tokio::task;
use tokio::sync::oneshot;
use crate::commands::utils::{handle_command_result, run_rcon_command};

use crate::Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn run_minecraft_command(args: String) -> Result<String, Error> {
    let directory = "steam/minecraft";
    run_rcon_command(vec![&args], directory).await
}

#[poise::command(slash_command)]
pub async fn minecraft_command(ctx: Context<'_>, command_param: String) -> Result<(), Error> {
    let (tx, rx) = oneshot::channel();
    let command_param_clone = command_param.clone();

    task::spawn(async move {
        let result = run_minecraft_command(command_param).await;
        let _ = tx.send(result);
    });

    let user_name = ctx.author().name.clone();
    ctx.say(format!("{} is Running {} Command", user_name, command_param_clone)).await?;

    if let Ok(result) = rx.await {
        handle_command_result(ctx, result).await?;
    }

    Ok(())
}