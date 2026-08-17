use crate::{Error, Context};

/// make pong
#[poise::command(
	slash_command
)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("pong").await?;
	Ok(())
}