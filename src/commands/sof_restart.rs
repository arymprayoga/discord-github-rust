use tokio::task;
use tokio::sync::oneshot;
use crate::Data;
use crate::commands::utils::run_docker_compose_command;
use crate::commands::utils::handle_command_result;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn run_docker_compose_restart() -> Result<String, Error> {
    run_docker_compose_command(vec!["compose", "down"]).await?;
    run_docker_compose_command(vec!["compose", "up", "-d"]).await
}

#[poise::command(slash_command)]
pub async fn sof_restart(ctx: Context<'_>) -> Result<(), Error> {
    let (tx, rx) = oneshot::channel();

    task::spawn(async move {
        let result = run_docker_compose_restart().await;
        let _ = tx.send(result);
    });

    ctx.say("Processing!").await?;

    if let Ok(result) = rx.await {
        handle_command_result(ctx, result).await?;
    }

    Ok(())
}