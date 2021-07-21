use std::env;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready, guild::Guild, id::ChannelId},
};

use crate::services::check_msg;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        // ctx.set_activity(Activity::playing())
        // .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if is_ignore_msg(&ctx, &msg).await {
            return;
        };

        let is_debug = false;
        if is_debug {
            debug_print(&msg, &ctx).await;
        };

        if msg.content.contains("http") {
            let link = msg.content.clone();
            check_msg(
                find_channel_id_by_name(&msg.guild(&ctx.cache).await.unwrap(), "links")
                    .say(&ctx.http, link)
                    .await,
            );
        };
    }
}

fn find_channel_id_by_name<'a>(guild: &Guild, name: &str) -> ChannelId {
    let map = guild.clone().channels;
    let mut name_map = map
        .iter()
        .map(|tuple| (tuple.0, String::from(&tuple.1.name)));
    let result = name_map.find_map(|(key, val)| if val == name { Some(key) } else { None });
    result.unwrap().clone()
}

async fn is_ignore_msg(_ctx: &Context, msg: &Message) -> bool {
    // botに反応しないようにする
    if msg.author.bot {
        return true;
    };

    // コマンドに反応しないようにする
    if msg.content.starts_with(
        &env::var("DISCORD_CMD_PREFIX").expect("Expected a command prefix in the environment"),
    ) {
        return true;
    };

    // voice channel にいない場合は動かさない
    // if get_handler_when_in_voice_channel(&ctx, &msg)
    //     .await
    //     .is_none()
    // {
    //     return true;
    // };

    false
}

async fn debug_print(msg: &Message, ctx: &Context) {
    // サーバーのID
    eprintln!("guild_id = {:?}", msg.guild_id);
    // チャンネル名
    let channel_name = msg.channel_id.name(&ctx.cache).await;
    eprintln!("channel_name = {:?}", channel_name);
    // メッセージの送信
    let content = msg.content.clone();
    println!("message received: {:?}", content);
}
