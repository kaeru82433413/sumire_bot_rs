use std::error::Error;
use serenity::{
    prelude::*,
    model::prelude::*,
    builder::CreateMessage,
};

use crate::consts;

pub async fn send_log(ctx: &Context, error: Box<impl Error + ?Sized>) {
    consts::ERROR_LOG_CHANNEL.send_message(ctx, |m| {
        m.content(format!("{:?}", error))
    }).await.unwrap();
}

pub async fn reply_to<F>(ctx: &Context, msg: &Message, f: F) -> serenity::Result<Message> where
    for<'a, 'b> F: FnOnce(&'b mut CreateMessage<'a>) -> &'b mut CreateMessage<'a>, 
{
    msg.channel_id.send_message(ctx, |m| {
        f(m);
        m.reference_message(msg).allowed_mentions(|a| {
            a.empty_parse()
        });
        m
    }).await
}