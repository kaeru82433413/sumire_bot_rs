use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::command,
    Args,
    CommandResult,
};
use serenity::utils::ArgumentConvert;
use diesel;
use diesel::prelude::*;
use crate::utils::*;


#[command]
#[aliases(cn_manage, coinm, cnm)]
#[sub_commands(add)]
async fn coin_manage(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "正しいサブコマンドが指定されませんでした").await?;
    Ok(())
}


#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);
    let target = args.string()?;
    let target = Member::convert(ctx, Some(crate::consts::SUMIRE_GUILD), None, &target).await?;
    let increase: i32 = args.parse()??;

    let conn = get_connection(ctx).await;
    let mut target_data = database::get_member_data(&conn, target.user.id.0 as i64)?;

    target_data.point += increase;
    diesel::update(&target_data).set(&target_data).execute(&conn)?;

    msg.reply(ctx, format!("所持コインに{}を加算しました。\n{}", increase,
        strings::PointTransition::new(&target).increase(increase).after(target_data.point)
        )).await?;

    Ok(())
}
