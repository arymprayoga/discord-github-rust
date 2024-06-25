use tokio::task;
use tokio::sync::oneshot;
use crate::commands::utils::{handle_command_result, run_docker_compose_command};

use crate::Data;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn run_docker_compose_up() -> Result<String, Error> {
    let directory = "steam/docker-sons-of-the-forest-dedicated-server";
    run_docker_compose_command(vec!["compose", "up", "-d"], directory).await
}

#[poise::command(slash_command)]
pub async fn sof_start(ctx: Context<'_>) -> Result<(), Error> {
    let (tx, rx) = oneshot::channel();

    task::spawn(async move {
        let result = run_docker_compose_up().await;
        let _ = tx.send(result);
    });

    ctx.say("Processing!").await?;

    if let Ok(result) = rx.await {
        handle_command_result(ctx, result).await?;
    }

    Ok(())
}