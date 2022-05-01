use serenity::builder::CreateEmbed;
use serenity::framework::standard::Command;
use crate::help::CommandList;

pub fn list(embed: &mut CreateEmbed, commands: &[&'static Command]) {
    for command in commands {
        embed.field(command.options.names[0], command.options.desc.unwrap(), false);
    }
}

pub fn detail(embed: &mut CreateEmbed, command: &'static Command, list: &CommandList) {
    let options = command.options;
    let full_name = list.full_name(command).join(" ");
    embed.title(&full_name);
    embed.description(options.desc.unwrap());
    
    if options.names.len() >= 2 {
        embed.field("エイリアス", options.names[1..].join(" "), false);
    }

    if !options.sub_commands.is_empty() {
        let sub_commands: Vec<_> = options.sub_commands.iter().map(|c| c.options.names[0]).collect();
        embed.field("サブコマンド", sub_commands.join(" "), false);
    }

    if let Some(usage) = options.usage {
        if !usage.is_empty() {
            embed.field("使い方", format!("{} {}", full_name, usage), false);
        }
    }

    if !options.examples.is_empty() {

        let mut examples = vec![];
        for example_args in options.examples.iter() {
            examples.push(format!("{} {}", full_name, example_args));
        }
        embed.field("例", examples.join("\n"), false);
    }
}