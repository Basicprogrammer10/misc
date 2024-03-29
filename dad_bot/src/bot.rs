use std::time::SystemTime;

use parking_lot::Mutex;
use rusqlite::Connection;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;

use crate::consts::DAD_TIMEOUT;
use crate::database::Database;
use crate::{commands, consts};

pub struct Bot {
    pub db: Mutex<Connection>,
}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, _ready: gateway::Ready) {
        Command::create_global_application_command(&ctx.http, |command| {
            commands::top_dads::register(command)
        })
        .await
        .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "top-dads" => commands::top_dads::run(&command.data.options),
                _ => "Command not found!?".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                eprintln!("Error handling command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let message = msg.content.to_lowercase();
        if consts::DAD_REGEX.is_match(&message) && check_dadding(self, &ctx, &msg).await {
            return;
        }

        if consts::IM_REGEX.is_match(&message) {
            if consts::AUTO_SHUT {
                if consts::SHUT_REGEX.is_match(&message) {
                    return;
                }

                println!(
                    "[*] Shutting message from `{}` in `{}`",
                    msg.author.name, msg.channel_id
                );
                msg.channel_id.say(&ctx.http, "(shut)").await.unwrap();
            } else {
                println!(
                    "[*] Added dadable from `{}` in `{}`",
                    msg.author.name, msg.channel_id
                );
                msg.react(&ctx.http, '👀').await.unwrap();
                self.db.lock().add_dadable(&msg).unwrap();
            }
        }

        async fn check_dadding(this: &Bot, ctx: &Context, msg: &Message) -> bool {
            let dadable = this
                .db
                .lock()
                .get_dadable(msg.guild_id.unwrap().0, msg.channel_id.0)
                .unwrap();

            let Some(dadable) = dadable else {
                return false;
            };

            let epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            if dadable.author_id == msg.author.id
                || (DAD_TIMEOUT > 0 && dadable.timestamp + DAD_TIMEOUT < epoch)
            {
                return false;
            }

            if let Ok(i) = msg.channel_id.message(&ctx.http, dadable.message_id).await {
                println!(
                    "[*] Added dad from `{}` on `{}` in `{}`",
                    msg.author.name, i.author.name, msg.channel_id
                );
                this.db.lock().add_daded(msg, &i).unwrap();
                i.react(&ctx.http, '🇱').await.unwrap();
                i.delete_reaction_emoji(&ctx.http, '👀').await.unwrap();
            }

            true
        }
    }
}
