use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::{group, command},
    Args,
    CommandResult,
};
use crate::utils::*;
use crate::help as help_crate;
use help_crate::{CommandListKey, CommandSearchResult};


#[command]
#[description("引数に指定したコマンドの詳細を表示します。引数を指定しなかった場合は、コマンドの一覧を表示します。")]
#[usage("[コマンド名]")]
#[example("coin")]
#[example("coin random")]
async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let list = data.get::<CommandListKey>().unwrap();

    let mut queries = vec![];
    while let Ok(query) = args.single::<String>() {
        queries.push(query);
    }

    if queries.is_empty() {
        reply_to(ctx, msg, |m| {
            m.embed(|e| {
                e.title("コマンドの一覧");
                help_crate::embed::list(e, list.roots());
                e
            })
        }).await?;

    } else {
        reply_to(ctx, msg, |m| {
            match list.search_from_root(&queries) {
                CommandSearchResult::Fond(command) => {
                    m.embed(|e| {
                        help_crate::embed::detail(e, command, list);
                        e
                    })
                },
                CommandSearchResult::NotParentCommand(command) => {
                    let full_name = list.full_name(command).join(" ");
                    m.content(format!("{}コマンドにサブコマンドはありません。\n{0}コマンドの詳細を表示しています。", full_name));
                    m.embed(|e| {
                        help_crate::embed::detail(e, command, list);
                        e
                    })
                },
                CommandSearchResult::RootCommandNotFound(query) => {
                    m.content(format!("{}というコマンドはありません。以下がコマンドの一覧です。", query));
                    m.embed(|e| {
                        help_crate::embed::list(e, list.roots());
                        e
                    })
                },
                CommandSearchResult::SubCommandNotFound(command, query) => {
                    let full_name = list.full_name(command).join(" ");
                    m.content(format!("{}コマンドに{}というサブコマンドはありません。\n{0}コマンドのサブコマンドの一覧を表示しています。", full_name, query));
                    m.embed(|e| {
                        help_crate::embed::list(e, command.options.sub_commands);
                        e
                    })
                }
            }
        }).await?;
    }

    Ok(())
}


#[group]
#[commands(help)]
pub struct Bot;
