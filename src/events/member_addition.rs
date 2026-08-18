use poise::serenity_prelude as serenity;
use crate::{Data, Error};

pub async fn member_addition(
    ctx: &serenity::Context,
    new_member: &serenity::Member
) -> Result<(), Error> {
    println!("new member: {}", new_member.user.name);
    println!("------------------------------------");
    println!("{:#?}", new_member);
    Ok(())
}

