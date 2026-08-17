use crate::{Data, Error};

mod ping;

pub fn register_commands() -> Vec<poise::Command<Data, Error>> {
	vec![
		ping::ping()
	]
}