use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    macros::{group, command},
    Args,
    CommandResult,
};
use crate::utils::*;
use fraction::Fraction;


#[command]
#[description("数式を計算します。")]
#[usage("<式>")]
async fn calc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg = args.message();

    if arg.is_empty() {
        return Err(args_wrapper::Eos.into());
    }

    let content = match expression::calc(&arg) {
        Ok(res) => format!("= {}", res),
        Err(err) => format!("{}", err),
    };
    msg.reply(ctx, content).await?;
    
    Ok(())
}


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


#[command]
#[description("引数の比を簡単にします。")]
#[usage("<値1> [値2]…")]
async fn ratio(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args = ArgsWrapper(args);

    let mut values = vec![];
    loop {
        let value = args.expression()?;
        if let Ok(value) = value {
            values.push(value);
        } else {
            break;
        }
    }

    if values.is_empty() {
        return Err(args_wrapper::Eos.into());
    }

    let mut mul = Some(1);
    let mut div = 0;
    for value in values.iter() {
        let (numer, denom) = (*value.numer().unwrap(), *value.denom().unwrap());
        div = num::integer::gcd(div, numer);
        mul = mul.and_then(|mul| {
            (mul/num::integer::gcd(mul, denom))
                .checked_mul(denom)
        });
    }

    let mut result = vec![];
    if let Some(mul) = mul {
        if div != 0 {
            for value in values.iter() {
                result.push((*value*Fraction::from(mul)/Fraction::from(div)).to_string());
            }
        } else {
            result = vec!["0".into(); values.len()];
        }
        msg.reply(ctx, result.join(" ")).await?;

    } else {
        msg.reply(ctx, "算術オーバーフローが発生しました。").await?;
    }

    
    Ok(())
}


#[group]
#[commands(calc, gcd, lcm, ratio)]
struct General;
