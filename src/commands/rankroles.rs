use deadpool_postgres::Pool;
use serenity::builder;
use serenity::model::prelude::command::{CommandOptionType};
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::Role;

use crate::models::rank_role::RankRole;
use crate::Handler;

pub async fn run(command: &ApplicationCommandInteraction, handler: &Handler) -> String {
    let subcommand = command
        .data
        .options
        .get(0)
        .expect("Expected a subcommand name");

    match subcommand.name.as_ref() {
        "create" => {
            let role_option = subcommand
                .options
                .get(0)
                .expect("Expected a role")
                .resolved
                .as_ref()
                .expect("Expected a role object");

            let level_option = subcommand
                .options
                .get(1)
                .expect("Expected a level")
                .resolved
                .as_ref()
                .expect("Expected a level value");

            let CommandDataOptionValue::Role(role) = role_option else { panic!("Invalid role option") };
            let CommandDataOptionValue::Integer(level) = level_option else { panic!("Invalid level option!") };
            let Some(guild_id) = command.guild_id else { panic!("Expected GuildId") };

            create(role, guild_id.0, level, &handler.db_pool).await
        }
        &_ => "Invalid subcommand :(".to_string(),
    }
}

pub async fn create(role: &Role, guild_id: u64, level: &i64, db: &Pool) -> String {
    let client = db.get().await.expect("Failed to get DB client");

    let result = client
        .query_one(
            "INSERT INTO miobot.rank_roles(level, role_id, guild_id) VALUES($1,$2,$3) RETURNING *",
            &[
                &(level.to_owned() as i32),
                &(role.id.0 as i64),
                &(guild_id as i64),
            ],
        )
        .await;
    let rank_role = RankRole::from(result.unwrap());

    format!(
        "trying to create {} at level {}",
        role.name, rank_role.level
    )
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("rankroles")
        .description("rankrole related commands")
        .create_option(|subcommand| {
            subcommand
                .name("create")
                .description("create a new rankrole")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|suboption| {
                    suboption
                        .name("role")
                        .kind(CommandOptionType::Role)
                        .required(true)
                        .description("Role to obtain at the set level")
                })
                .create_sub_option(|suboption| {
                    suboption
                        .name("level")
                        .kind(CommandOptionType::Integer)
                        .required(true)
                        .description("Level to obtain the role at")
                })
        })
}
