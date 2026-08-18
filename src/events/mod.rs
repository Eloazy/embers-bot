use poise::serenity_prelude as serenity;
use crate::{Data, Error};

mod member_addition;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
	    serenity::FullEvent::GuildMemberAddition { new_member } => {
	        member_addition::member_addition(ctx, new_member).await?;
	    }
	    _ => {}
	}
    Ok(())
}