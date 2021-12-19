use dicer::eval;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
bah humbug...
";

const HELP_COMMAND: &str = "!rusty";
const ROLL_COMMAND: &str = "!d";
const HISTORY_COMMAND: &str = "!history";

struct RollHistory;

impl TypeMapKey for RollHistory {
    type Value = HashMap<String, Vec<String>>;
}

async fn reg(ctx: &Context, name: &str, expr: &str) {
    let mut data = ctx.data.write().await;
    let history = data.get_mut::<RollHistory>().unwrap();
    let entry = history.entry(name.to_string()).or_insert_with(Vec::new);

    if entry.len() < 10 {
        entry.push(String::from(expr));
    } else {
        while entry.len() >= 10 {
            entry.remove(0);
        }
        entry.push(String::from(expr));
    }
}

async fn history_command(ctx: &Context, msg: &Message) {
    let author = &msg.author.name;
    let data = ctx.data.read().await;
    let history = data.get::<RollHistory>().unwrap();
    let entry = history.get(author);
    if let Some(user_rolls) = entry {

        let content = msg.content.replace(HISTORY_COMMAND, "").replace(" ", "");
        let index:i32 = content.parse::<i32>().unwrap_or(-1);

        if index >= 0 && index <= user_rolls.len().try_into().unwrap() {
            let result = eval(&user_rolls[(index - 1) as usize]);
            match result {
                Ok(s) => {
                    let mut response = String::from("```yaml\n");
                    for res in s {
                        response.push_str(format!("{} = {}\n", res.str, res.value).as_str());
                    }
                    response.push_str("```");

                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending message: {:?}", why);
                    }
                    return
                }
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "nah").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        }

        let mut response = String::from("```yaml\n");
        for (pos, roll) in user_rolls.iter().enumerate() {
            response.push_str(format!("{}:{}\n", pos + 1, roll).as_str());
        }
        response.push_str("```");
        if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with(ROLL_COMMAND) {
            let author = msg.author.name;

            let content: Vec<&str> = msg.content.split(ROLL_COMMAND).collect();
            let input_str = String::from(content[1]);
            println!("{} : {}", author, input_str);

            let result = eval(&input_str);
            match result {
                Ok(s) => {
                    let mut response = String::from("```yaml\n");
                    for res in s {
                        response.push_str(format!("{} = {}\n", res.str, res.value).as_str());
                    }
                    response.push_str("```");

                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        println!("Error sending message: {:?}", why);
                    }

                    reg(&ctx, &author, &input_str).await;
                }
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "nah").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }
        } else if msg.content.starts_with(HISTORY_COMMAND) {
            history_command(&ctx, &msg).await;
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<RollHistory>(HashMap::default());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
