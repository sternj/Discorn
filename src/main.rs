use rand::{thread_rng, Rng};
use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::prelude::EventHandler;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Mutex;

struct DiscornHandler {
    words: Vec<String>,
    curr_word: Mutex<String>,
}

fn get_random_word(words: &Vec<String>) -> String {
    let mut rng = thread_rng();
    let idx = rng.gen_range(0, words.len());
    return String::from(&words[idx]);
}

impl DiscornHandler {
    fn new(path: PathBuf) -> DiscornHandler {
        let f = File::open(path).unwrap();
        let lines = BufReader::new(f).lines();
        let words: Vec<String> = lines.filter_map(Result::ok).collect();
        let initial_str = get_random_word(&words);
        println!("the first corn word is {}", initial_str);
        return DiscornHandler {
            words: words,
            curr_word: Mutex::new(initial_str),
        };
    }
}

impl EventHandler for DiscornHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let mut match_string = self.curr_word.lock().unwrap();
        let m = match_string.to_string().to_lowercase();
        if msg.content.contains(&m.to_lowercase()) {
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!(
                    ":corn::corn::corn: YOU SAID THE CORN WORD! {} :corn::corn::corn:",
                    &m
                ),
            ) {
                println!("Error sending message: {}", why)
            }
            let new_str = get_random_word(&self.words);
            *match_string = new_str;
        }
    }
}

fn main() {
    let handler = DiscornHandler::new(PathBuf::from("./words.txt"));
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Token"), handler)
        .expect("Could not initialize client");
    if let Err(why) = client.start() {
        println!("Client error: {}", why)
    }
}
