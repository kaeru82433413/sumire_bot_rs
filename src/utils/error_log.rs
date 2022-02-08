use std::error::Error;
use serenity::prelude::*;
use crate::consts;

pub async fn send_log(ctx: &Context, error: Box<impl Error + ?Sized>) {
    consts::ERROR_LOG_CHANNEL.send_message(ctx, |m| {
        m.content(format!("{:?}", error))
    }).await.unwrap();
}