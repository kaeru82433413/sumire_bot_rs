use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::{group, command},
    Args,
    CommandResult,
};
use diesel;
use diesel::prelude::*;
use crate::utils::*;

#[command]
async fn sql(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let conn = get_connection(ctx).await;
    let query = args.message();
    let res = diesel::sql_query(query).execute(&conn)?;
    msg.reply(&ctx, format!("Result rows: {:}", res)).await?;
    Ok(())
}

mod coin_manage;
use coin_manage::COIN_MANAGE_COMMAND;

#[group]
#[owners_only]
#[commands(sql, coin_manage)]
struct Owner;