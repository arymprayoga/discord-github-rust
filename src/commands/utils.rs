use tokio::process::Command;
use std::error::Error;
use poise::Context;
use crate::Data;
use dirs;

pub async fn run_docker_compose_command(args: Vec<&str>, directory: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut dir = dirs::home_dir().expect("Home directory not found");
    dir.push(directory);

    match Command::new("docker")
        .args(&args)
        .current_dir(dir)
        .output()
        .await {
        Ok(output) => {
            if output.status.success() {
                Ok("Command executed successfully".to_string())
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to execute command: {}", err).into())
            }
        }
        Err(e) => {
            Err(format!("Error running command: {}", e).into())
        }
    }
}

pub async fn run_rcon_command(args: Vec<&str>, directory: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut dir = dirs::home_dir().expect("Home directory not found");
    dir.push(directory);

    match Command::new("docker")
        .args(&["exec", "minecraft-mc-1", "rcon-cli"])
        .args(&args)
        .current_dir(dir)
        .output()
        .await {
        Ok(output) => {
            if output.status.success() {
                Ok("Command executed successfully".to_string())
            } else {
                let err = String::from_utf8_lossy(&output.stderr);
                Err(format!("Failed to execute command: {}", err).into())
            }
        }
        Err(e) => {
            Err(format!("Error running command: {}", e).into())
        }
    }
}

pub async fn handle_command_result(ctx: Context<'_, Data, Box<dyn Error + Send + Sync>>, result: Result<String, Box<dyn Error + Send + Sync>>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match result {
        Ok(success_message) => {
            ctx.say(success_message).await?;
        }
        Err(error) => {
            ctx.say(error.to_string()).await?;
        }
    }
    Ok(())
}