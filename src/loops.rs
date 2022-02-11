use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::framework::standard::CommandResult;
use serenity::model::id::RoleId;
use chrono::{Local, NaiveDateTime, NaiveDate, Timelike, Datelike, Weekday};
use crate::utils::error_log::send_log;
use tokio::{self, time::Duration};
use crate::consts;
use diesel::prelude::*;
use crate::schema::point_record::dsl::point_record;
use crate::model::{Member as MemberData, PointRecordForm};
use crate::utils::*;

const SEICHI_ACHIEV_ROLE: RoleId = RoleId(876675066329432114);


pub async fn start(ctx: Context) {
    loop {
        let time = Local::now().time();
        let wait_for = 60.0 - (time.second() as f64 + time.nanosecond() as f64 * 1e-9);
        tokio::time::sleep(Duration::from_secs_f64(wait_for)).await;

        let now = Local::now();
        eprintln!("{:?}", now);
        if let Err(why) = minutely(&ctx, &now.naive_local()).await {
            send_log(&ctx, why).await;
        }
    }
}

async fn minutely(ctx: &Context, now: &NaiveDateTime) -> CommandResult {
    if now.time().minute() == 0 {
        hourly(ctx, now).await?;
    }
    Ok(())
}

async fn hourly(ctx: &Context, now: &NaiveDateTime) -> CommandResult {
    match now.time().hour() {
        0 => daily(ctx, &now.date()).await?,
        22 => daily22(ctx, &now.date()).await?,
        _ => (),
    }
    Ok(())
}

async fn daily(ctx: &Context, today: &NaiveDate) -> CommandResult {
    let conn = get_connection(ctx).await;

    let members: Vec<_> = consts::SUMIRE_GUILD.members(ctx, None, None).await?
        .into_iter().filter(|m| !m.user.bot).collect();
    let members_data = database::get_members_data(&conn, &members)?;

    for MemberData { id, point, .. } in members_data {
        let form = PointRecordForm {
            id, point,
            date: *today,
        };
        diesel::insert_into(point_record).values(form).execute(&conn)?;
    }

    let achieves = search_today_achievements(today);
    if !achieves.is_empty() {
        let (mut id_list, mut name_list) = (String::new(), String::new());
        for (i, (id, name)) in achieves.iter().enumerate() {
            if i > 0 {
                id_list.push_str(",");
                name_list.push_str("と");
            }
            id_list.push_str(&id.to_string());
            name_list.push_str(&format!("「{}」", name));
        }

        consts::REGULAR_NOTIFICATION_CHANNEL.send_message(ctx, |m| {
            m.content(MessageBuilder::new().mention(&SEICHI_ACHIEV_ROLE))
             .embed(|e| {
                e.title("整地鯖の記念日実績が解除できます")
                  .description(format!("本日は {}です\n実績No{}を解除していない人は忘れずに解除しましょう",
                                       name_list, id_list))
                  .color(0xffff00)
            })
        }).await?;
    }

    Ok(())
}

async fn daily22(ctx: &Context, today: &NaiveDate) -> CommandResult {
    if !search_today_achievements(today).is_empty() {
        consts::REGULAR_NOTIFICATION_CHANNEL.say(ctx, 
            MessageBuilder::new().mention(&SEICHI_ACHIEV_ROLE).push(
                "\n整地鯖記念日実績の解除をお忘れではありませんか？年に一度の機会なので、解除し忘れないようにしましょう")).await?;
    }

    Ok(())
}


fn search_today_achievements(today: &NaiveDate) -> Vec<(i32, &str)> {
    let mut res = vec![];

    res.extend(match (today.month(), today.day()) {
        (1, 1) => vec![(9001, "とある始まりの日")],
        (2, 3) => vec![(9006, "とあるお豆の絨毯爆撃の日")],
        (2, 11) => vec![(9007, "建国記念日")],
        (2, 14) => vec![(9008, "とあるカカオまみれの日")],
        (3, 3) => vec![(9010, "とある女の子の日")],
        (3, 14) => vec![(9011, "燃え尽きたカカオだらけの日")],
        (4, 1) => vec![(9014, "とある嘘の日")],
        (4, 15) => vec![(9015, "とある良い子の日")],
        (4, 22) => vec![(9016, "とある掃除デー")],
        (5, 5) => vec![(9018, "とある子供の日"), (9019, "端午の節句")],
        (6, 12) => vec![(9022, "とある日記の日")],
        (6, 29) => vec![(9024, "とある生誕の日")],
        (7, 7) => vec![(9026, "七夕")],
        (7, 17) => vec![(9027, "とある東京の日")],
        (7, 29) => vec![(9028, "とある肉の日")],
        (8, 7) => vec![(9030, "とあるバナナの日")],
        (8, 21) => vec![(9031, "とあるJDの日")],
        (8, 29) => vec![(9032, "とある焼肉の日")],
        (9, 2) => vec![(9034, "とあるくじの日")],
        (9, 12) => vec![(9035, "とあるマラソンの日")],
        (9, 15) => vec![(9039, "とある月見の日")],
        (9, 21) => vec![(9037, "とある中秋の日"), (9038, "とあるファッションショーの日")],
        (9, 29) => vec![(9036, "とあるふぐの日")],
        (10, 10) => vec![(9041, "とあるスポーツの日")],
        (11, 15) => vec![(9043, "とある七五三の日")],
        (11, 29) => vec![(9044, "とある特上の肉の日")],
        (12, 1) => vec![(9046, "とある年の暮れの日")],
        (12, 25) => vec![(9002, "とある聖夜の日"), (9047, "とあるクリスマスの日")],
        (12, 31) => vec![(9003, "とある終わりの日")],
        _ => vec![],
    });

    res.extend(match (today.month(), (today.day()-1)/7+1, today.weekday()) {
        (5, 2, Weekday::Sun) => vec![(9020, "母の日")],
        (6, 3, Weekday::Sun) => vec![(9023, "父の日")],
        _ => vec![],
    });

    if (today.month(), today.day()) == (3, (20.8431 + 0.242194 * (today.year() as f64 - 1980.0) - ((today.year()-1980)/4) as f64).floor() as u32) {
        res.push((3012, "春分の日"));
    }

    res
}