use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::{group, command},
    Args,
    CommandResult,
};
use crate::utils::*;



#[command]
#[description("GCD(最大公約数)を求めます。")]
#[usage("[値1] [値2]…")]
async fn gcd(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);

    let mut values = vec![];
    loop {
        let value = args.parse::<i64>()?;
        if let Ok(value) = value {
            values.push(value)
        } else {
            break;
        }
    }

    let mut result = Some(0);
    for value in values {
        result = if let (Some(result), Some(value_abs)) = (result, value.checked_abs()) {
            Some(num::integer::gcd(result, value_abs))
        } else {None};
    }

    let content = match result {
        Some(value) => format!("= {}", value),
        None => "値が大きすぎて計算できませんでした。".into(),
    };
    msg.reply(ctx, content).await?;
    
    Ok(())
}


#[command]
#[description("LCM(最小公倍数)を求めます。")]
#[usage("[値1] [値2]…")]
async fn lcm(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);

    let mut values = vec![];
    loop {
        let value = args.parse::<i64>()?;
        if let Ok(value) = value {
            values.push(value)
        } else {
            break;
        }
    }

    let mut result = Some(1);
    for value in values {
        result = result.and_then(|mut result| {
            result /= num::integer::gcd(result, value);
            result = result.checked_mul(value)?.checked_abs()?;
            Some(result)
        });
    }

    let content = match result {
        Some(value) => format!("= {}", value),
        None => "値が大きすぎて計算できませんでした。".into(),
    };
    msg.reply(ctx, content).await?;
    
    Ok(())
}




#[group]
#[commands(gcd, lcm)]
struct General;
