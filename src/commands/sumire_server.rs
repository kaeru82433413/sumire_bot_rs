use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::{group, command},
    Args,
    CommandResult,
};
use serenity::utils::{MessageBuilder, ArgumentConvert};

use crate::utils::*;
use crate::consts::*;
use crate::model::Member as MemberModel;
use diesel::prelude::*;
use std::collections::HashMap;
use rand::prelude::*;
use chrono::{Local, Duration};
use once_cell::sync::Lazy;

static ROLES: Lazy<HashMap<RoleId, &str>> = Lazy::new(|| {
    HashMap::from([
        (876675066329432114.into(), "<#820939592999108648>で整地鯖の記念日実績を通知します。"),
        (941684228624633896.into(), "お遊びロールです。破産したら付けてみましょう。")
    ])
});


#[command]
#[aliases("pt")]
#[sub_commands(ranking, transfer, random, daily)]
async fn point(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "正しいサブコマンドが指定されませんでした").await?;
    Ok(())
}

#[command]
async fn ranking(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);
    let page: usize = args.parse()?.unwrap_or(1); // 何ページ目か(1-based)

    let members: Vec<_> = SUMIRE_GUILD.members(ctx, None, None).await?
        .into_iter().filter(|m| !m.user.bot).collect();
    let pages = (members.len()+9)/10;
    if !(1 <= page && page <= pages) {
        msg.reply(ctx, format!("{}ページ目は存在しません", page)).await?;
        return Ok(());
    }
    let page = page-1; // 0-based

    let conn = get_connection(ctx).await;
    let mut members_data = database::get_members_data(&conn, &members)?;
    members_data.sort_by_key(|m| (-m.point, m.id)); // pointの大きい順。idの小さい順。
    
    let mut description = MessageBuilder::new();
    let members_map: HashMap<_, _> = members.into_iter().map(|m| (m.user.id.0 as i64, m)).collect();

    let mut prev = (1, i32::MAX);
    for (i, MemberModel { id, point, .. }) in members_data.into_iter().enumerate() {
        if point < prev.1 {
            prev = (i, point)
        }
        let rank = prev.0+1;

        if !(10*page <= i && i < 10*page+10) {
            continue;
        }

        if i%10!=0 {
            description.push("\n");
        }
        description.push(&format!("{}位 {}pt: ", rank, point))
            .mention(&members_map[&id]);
    }

    let title = format!("ポイントランキング ({}/{}ページ)", page+1, pages);
    reply_to(ctx, msg, |m| {
        m.embed(|e| {
            e.title(title)
             .description(description)
        })
    }).await?;
    Ok(())
}

#[command]
async fn transfer(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);
    let target = args.string()?;
    let target = Member::convert(ctx, Some(SUMIRE_GUILD), None, &target).await?;
    let value: i32 = args.parse()??;

    if target.user.bot {
        msg.reply(ctx, "botには譲渡できません").await?;
        return Ok(());
    }

    if value <= 0 {
        msg.reply(ctx, "自然数を指定してください").await?;
        return Ok(());
    }

    let conn = get_connection(ctx).await;

    let mut executer_data = database::get_member_data(&conn, msg.author.id.0 as i64)?;
    let mut target_data = database::get_member_data(&conn, target.user.id.0 as i64)?;

    if executer_data.point < value {
        msg.reply(ctx, format!("所持ポイントが足りないため実行できません(所持ポイント:{})", executer_data.point)).await?;
        return Ok(());
    }

    executer_data.point -= value;
    target_data.point += value;
    diesel::update(&executer_data).set(&executer_data).execute(&conn)?;
    diesel::update(&target_data).set(&target_data).execute(&conn)?;

    let target_name = strings::safe(target.nick.as_ref().unwrap_or(&target.user.name));
    msg.reply(ctx, format!("{}に{}ポイント譲渡しました\n{}\n{}", &target_name, value,
        strings::point_transition(strings::display_name(msg), executer_data.point+value, executer_data.point),
        strings::point_transition(&target_name, target_data.point-value, target_data.point))).await?;
    Ok(())
}

#[command]
async fn random(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let value: i32 = ArgsWrapper(args).parse()??;
    let conn = get_connection(ctx).await;

    if value <= 0 {
        msg.reply(ctx, "自然数を指定してください").await?;
        return Ok(());
    }
    
    let mut data = database::get_member_data(&conn, msg.author.id.0 as i64)?;
    let before_point = data.point;
    if before_point < value {
        msg.reply(ctx, format!("所持ポイントが足りないため実行できません(所持ポイント:{})", data.point)).await?;
        return Ok(());
    }

    let mut reply_message = MessageBuilder::new();
    if rand::random() {
        data.point += value;
        reply_message.push("おめでとう！あたり！\n");
    } else {
        data.point -= value;
        reply_message.push("残念！はずれ！\n");
    };
    reply_message.push(strings::point_transition(strings::display_name(msg), before_point, data.point));

    diesel::update(&data).set(&data).execute(&conn)?;
    msg.reply(ctx, reply_message).await?;
    Ok(())
}


#[command]
async fn daily(ctx: &Context, msg: &Message) -> CommandResult {
    let conn = get_connection(ctx).await;
    let mut data = database::get_member_data(&conn, msg.author.id.0 as i64)?;

    let now = Local::now().naive_local();
    let today = (now-Duration::hours(4)).date();

    if data.last_daily >= today {
        let next = (today+Duration::days(1)).and_hms(4, 0, 0).format("%m/%d-%H:%S");
        msg.reply(ctx, format!("{}の分は既に受け取っています。{}以降に再度使用してください",
            today.format("%m/%d"), next)).await?;
        return Ok(());
    }

    let before_point = data.point;
    let value = { // ThreadRngは!Sendなのですぐにdropさせる
        let mut rng = thread_rng();
        if rng.gen::<f64>() < 0.01 {256} else {
            random::round(11744.0/99.0 + rng.gen_range(-40.0..=40.0))
            // 11744/99(≒118.62)は方程式(256+99*x)/100=120の解
        }
    };
    data.point += value;
    data.last_daily = today;
    diesel::update(&data).set(&data).execute(&conn)?;
    msg.reply(ctx, format!("デイリーボーナスを受け取りました！{}ptゲット！\n{}", value, strings::point_transition(strings::display_name(msg), before_point, data.point))).await?;
    Ok(())
}


#[command]
#[sub_commands(list, add, remove)]
async fn role(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx, "正しいサブコマンドが指定されませんでした").await?;
    Ok(())
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_roles = SUMIRE_GUILD.roles(ctx).await?;
    reply_to(ctx, msg, |m| {
        m.embed(|e| {
            e.title("着脱可能なロールの一覧");
            for (id, role_info) in ROLES.iter() {
                let role = &guild_roles[id];
                e.field(&role.name, role_info, false);
            }
            e
        })
    }).await?;
    Ok(())
}

#[command]
async fn add(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut member = msg.member(ctx).await?;

    let role_name = ArgsWrapper(args).string()?;
    let role = Role::convert(ctx, Some(SUMIRE_GUILD), None, &role_name).await?;

    if !ROLES.contains_key(&role.id) {
        msg.reply(ctx, "そのロールの操作はできません").await?;
        return Ok(());
    }

    if member.roles.contains(&role.id) {
        msg.reply(ctx, "既に付与されています").await?;
    } else {
        member.add_role(ctx, role.id).await?;
        msg.reply(ctx, format!("{}を付与しました", role.name)).await?;
    }

    Ok(())
}

#[command]
async fn remove(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut member = msg.member(ctx).await?;

    let role_name = ArgsWrapper(args).string()?;
    let role = Role::convert(ctx, Some(SUMIRE_GUILD), None, &role_name).await?;

    if !ROLES.contains_key(&role.id) {
        msg.reply(ctx, "そのロールの操作はできません").await?;
        return Ok(());
    }

    if !member.roles.contains(&role.id) {
        msg.reply(ctx, "そのロールは付与されていません").await?;
    } else {
        member.remove_role(ctx, role.id).await?;
        msg.reply(ctx, format!("{}を削除しました", role.name)).await?;
    }

    Ok(())
}


#[group]
#[commands(point, role)]
struct SumireServer;
