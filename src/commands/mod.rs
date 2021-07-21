use std::{sync::Arc, time::Duration};

use serenity::{
    async_trait,
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    http::Http,
    model::{channel::Message, misc::Mentionable, prelude::ChannelId},
};

use crate::services::check_msg;

#[group]
#[commands(ping, help)]
pub(crate) struct General;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(
        msg.channel_id
            .say(&ctx.http, "#links にリンクを転送するよ")
            .await,
    );
    Ok(())
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "Pong!").await);

    Ok(())
}
