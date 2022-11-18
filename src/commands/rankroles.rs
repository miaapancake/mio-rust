use serenity::builder;
use serenity::model::prelude::Role;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};

pub fn run(options: &[CommandDataOption]) -> String {
    let subcommand = options
        .get(0)
        .expect("Expected a subcommand name");
    

    match subcommand.name.as_ref() {
        "create" => {
            let role_option = subcommand.options.get(0).expect("Expected a role").resolved.as_ref().expect("Expected a role object");
            let level_option = subcommand.options.get(1).expect("Expected a level").resolved.as_ref().expect("Expected a level value");
            
            let CommandDataOptionValue::Role(role) = role_option else { panic!("Invalid role option") };
            let CommandDataOptionValue::Integer(level) = level_option else { panic!("Invalid level option!") };

            create(role, level)
        },
        &_ => "Invalid subcommand :(".to_string()
    }
}

pub fn create(role: &Role, level: &i64) -> String {
    format!("trying to create {} at level {}", role.name, level)
}

pub fn register(
    command: &mut builder::CreateApplicationCommand
) -> &mut builder::CreateApplicationCommand {
    command
        .name("rankroles")
        .description("rankrole related commands")
        .create_option(|subcommand| subcommand.name("create").description("create a new rankrole")
            .kind(CommandOptionType::SubCommand)
                .create_sub_option(|suboption| suboption.name("role").kind(CommandOptionType::Role).required(true).description("Role to obtain at the set level"))
                .create_sub_option(|suboption| suboption.name("level").kind(CommandOptionType::Integer).required(true).description("Level to obtain the role at"))
        )
}