use rand::{thread_rng, Rng};
use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::model::guild::Guild;
use serenity::model::id::GuildId;
use serenity::prelude::EventHandler;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cell::Cell;
use std::path::PathBuf;
use serenity::prelude::Mutex;
use std::sync::RwLock;
#[derive(Debug)]
struct Word {
    light_word: String,
    dark_word: String,
}

struct DiscornGuild {
    dark_words: Vec<String>,
    light_word: String,
    dark_word: String
}

struct DiscornHandler {
    default_dark_words: Vec<String>,
    light_words: Vec<String>,
    guilds: RwLock<HashMap<GuildId, Mutex<DiscornGuild>>>
}

fn get_random_word(words: &Vec<String>) -> String {
    let mut rng = thread_rng();
    let idx = rng.gen_range(0, words.len());
    return String::from(&words[idx]);
}

impl DiscornHandler {
    fn new(light_path: PathBuf, dark_path: PathBuf) -> DiscornHandler {
        let f = File::open(light_path).unwrap();
        let f2 = File::open(dark_path).unwrap();
        let lines = BufReader::new(f).lines();
        let dark_lines = BufReader::new(f2).lines();
        let words: Vec<String> = lines.filter_map(Result::ok).collect();
        let dark_words: Vec<String> = dark_lines.filter_map(Result::ok).collect();
        //let initial_str = get_random_word(&words);
        //println!("the first corn word is {}", initial_str);
        let word_struct = Word {
            light_word: get_random_word(&words),
            dark_word: get_random_word(&dark_words),
        };
        println!("{:?}", word_struct);
        return DiscornHandler {
            light_words: words,
            default_dark_words: dark_words,
            guilds: RwLock::new(HashMap::new())
        };
    }

    fn add_guild(&self, guild_id: GuildId) {
        let mut hm = self.guilds.write().unwrap();
        let dark_words = self.default_dark_words.clone();
        hm.insert(guild_id, Mutex::new(DiscornGuild {
            light_word: get_random_word(&self.light_words),
            dark_word: get_random_word(&dark_words),
            dark_words: dark_words,
        }));
    }
}

impl EventHandler for DiscornHandler {
    fn cache_ready(&self, _ctx: Context, guilds: Vec<GuildId>) {
        for guild_id in guilds {
            self.add_guild(guild_id);
        }
    }
    fn guild_create(&self, _ctx: Context, guild: Guild, _is_new: bool) {
        self.add_guild(guild.id);
    }
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.read().user.id {
            return;
        }
        let guild_id = match msg.guild_id {
            Some(e) => e,
            None => {
                report_error(&msg, &ctx, "Message must be sent in gateway");
                return;
            }
        };
        let guild_rw = match self.guilds.read() {
            Ok(o) => o,
            Err(e) => {
                report_error(&msg, &ctx, e);
                return;
            },
        };
        let guild = guild_rw.get(&guild_id).unwrap();
        let mut guild_mx = guild.lock();
        let rnd: f32 = thread_rng().gen_range(0f32, 1f32);
        if msg
            .content
            .to_lowercase()
            .contains(&guild_mx.light_word.to_lowercase())
        {
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!(
                    ":corn::corn::corn: YOU SAID THE CORN WORD! {} :corn::corn::corn:",
                    &guild_mx.light_word
                ),
            ) {
                println!("Error sending message: {}", why)
            }
            if rnd < 0.25 {
                if let Err(why) = msg.channel_id.say(
                    &ctx.http,
                    "Checkmate atheists"
                ) {
                    println!("Error sending message: {}", why);
                }
            } 
            let new_str = get_random_word(&self.light_words);
            guild_mx.light_word = new_str;
        }
        else if msg
            .content
            .to_lowercase()
            .contains(&guild_mx.dark_word.to_lowercase())
        {
            //let rnd: f32 = thread_rng().gen_range(0f32, 1f32);
            //if rnd < 0.75f32 {
                if let Err(why) = msg.channel_id.say(
                    &ctx.http,
                    format!(
                        ":corn::corn::corn: YOU SAID THE CORN WORD! {} :corn::corn::corn:",
                        &guild_mx.dark_word
                    ),
                ) {
                    println!("Error sending message: {}", why)
                }
                
                if rnd < 0.25 {
                    if let Err(why) = msg.channel_id.say(
                        &ctx.http,
                        "Checkmate atheists"
                    ) {
                        println!("Error sending message: {}", why);
                    }
                } 
                let new_str = get_random_word(&guild_mx.dark_words);
                guild_mx.dark_word = new_str;
            //}
        }

        
    }
}

fn report_error<T: std::fmt::Display>(msg: &Message, ctx: &Context, e: T) {
    if let Err(ee) = msg.channel_id.say(
        &ctx.http,
        format!("There was an error: Please report the following to sam@samstern.me\n {}", e)
    ) {
        println!("Error trying to report error {}: {}", e, ee);
    }
}

fn main() {
    let handler = DiscornHandler::new(
        PathBuf::from("./light-corn-words.txt"),
        PathBuf::from("./dark-corn-words.txt"),
    );
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Token"), handler)
        .expect("Could not initialize client");
    if let Err(why) = client.start() {
        println!("Client error: {}", why)
    }
}
