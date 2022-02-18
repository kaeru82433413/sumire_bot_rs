use std::env;
use std::collections::HashSet;
use std::num::{ParseIntError, IntErrorKind};
use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    framework::standard::{
        macros::hook,
        CommandResult,
        StandardFramework,
    },
    model::{
        channel::{Message},
        gateway::Ready,
        id::*,
    },
};
use sumire_bot::loops;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready.", ready.user.name);
        if consts::IS_DEBUG {
            let channel = ChannelId(769174683227521064);
            channel.say(&ctx, format!("{} is ready.", ready.user.name)).await.unwrap();
        } else {
            consts::LOGIN_NOTIFICATION_CHANNEL.say(&ctx, format!("{} がログインしたよ！", ready.user.tag())).await.unwrap();
        }
        loops::start(ctx).await;
    }
}


#[hook]
async fn after(ctx: &Context, msg: &Message, command_name: &str, command_result: CommandResult) {
    let err = match command_result {
        Ok(()) => return,
        Err(err) => err,
    };

    if let Some(_) = err.downcast_ref::<args_wrapper::Eos>() {
        msg.channel_id.say(&ctx, "引数が足りません").await.unwrap();

    } else if let Some(args_wrapper::ParseError::<ParseIntError>(raw, error)) = err.downcast_ref() {
        let reason = match error.kind() {
                IntErrorKind::Empty => "空文字列です",
                IntErrorKind::InvalidDigit => "無効な文字が含まれています",
                IntErrorKind::PosOverflow => "値が大きすぎます",
                IntErrorKind::NegOverflow => "値が小さすぎます",
                IntErrorKind::Zero => "0は使用できません",
                _ => unreachable!(),
            };
        msg.channel_id.say(&ctx, format!(r#"引数として与えられた"{}"を整数に変換できませんでした({})"#, strings::safe(raw), reason)).await.unwrap();
        
    } else {
        println!("{} でエラーが発生しました: {:?}", command_name, err);
        error_log::send_log(ctx, err).await;
    }
}

use sumire_bot::commands::*;
use sumire_bot::consts;
use sumire_bot::utils::*;

struct PrefixesKey;
impl TypeMapKey for PrefixesKey {
    type Value = Vec<&'static str>;
}


#[tokio::main]
async fn main() {
    let token = env::var("SUMIRE_BOT_TOKEN").expect("環境変数の取得に失敗しました");

    let prefixes = if consts::IS_DEBUG {vec!["?"]} else {vec!["s/", "!"]};
    async fn starts_with_prefix(ctx: &Context, content: &str) -> bool {
        let data = ctx.data.read().await;
        let prefixes = data.get::<PrefixesKey>().unwrap();
        for prefix in prefixes {
            if content.starts_with(prefix) {
                return true;
            }
        }
        false
    }
    let delimiters = consts::WHITESPACES.into_iter().map(|&c| char::from_u32(c).unwrap());
    let owners = HashSet::from([481027469202423808.into()]);

    let framework = StandardFramework::new()
        .configure(|c| c.prefixes(&prefixes).delimiters(delimiters).owners(owners)
            .dynamic_prefix(|ctx, msg| {
                Box::pin(async move {
                    if !consts::IS_DEBUG  && !starts_with_prefix(ctx, &msg.content).await
                        && msg.channel_id.name(ctx).await.unwrap().contains("コマンド")
                        {Some("".into())} else {None}
                })
            }))
        .group(&SUMIRESERVER_GROUP)
        .group(&OWNER_GROUP)
        .after(after);

    
    let mut client = 
        Client::builder(&token).event_handler(Handler).framework(framework).intents(GatewayIntents::all())
        .application_id(769157262697824256).await.expect("クライアントの作成に失敗しました");
    {
        let mut data = client.data.write().await;
        data.insert::<PrefixesKey>(prefixes);
    }
    database::insert_pool(&client).await;
    
    if let Err(why) = client.start().await {
        println!("{}", why);
    }
}
