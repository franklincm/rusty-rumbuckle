use dicer::eval;
use std::collections::HashMap;
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

struct RollHistory;

impl TypeMapKey for RollHistory {
    type Value = HashMap<String, Vec<String>>;
}

async fn reg(ctx: &Context, name: &String, expr: &String) {
    let mut data = ctx.data.write().await;
    let history = data.get_mut::<RollHistory>().unwrap();
    let entry = history
        .entry(name.to_string())
        .or_insert(Vec::new());
    entry.push(String::from(expr));
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

            reg(&ctx, &author, &input_str).await;

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
                }
                Err(_) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "nah").await {
                        println!("Error sending message: {:?}", why);
                    }
                }
            }

            let data = ctx.data.read().await;
            let history = data.get::<RollHistory>().unwrap();
            let entry = history.get(&author);
            match entry {
                Some(lookup) => println!("HISTORY: {} = {}", author, lookup[0]),
                None => (),
            }
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
