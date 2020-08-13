use rand::{thread_rng, Rng};
use serenity::client::{Client, Context};
use serenity::model::channel::Message;
use serenity::prelude::EventHandler;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug)]
struct Word {
    light_word: String,
    dark_word: String,
}

struct DiscornHandler {
    light_words: Vec<String>,
    dark_words: Vec<String>,
    curr_word: Mutex<Word>,
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
            dark_words: dark_words,
            curr_word: Mutex::new(word_struct),
        };
    }
}

impl EventHandler for DiscornHandler {
    fn message(&self, ctx: Context, msg: Message) {
        let mut words = self.curr_word.lock().unwrap();
        
        if msg
            .content
            .to_lowercase()
            .contains(&words.light_word.to_lowercase())
        {
            if let Err(why) = msg.channel_id.say(
                &ctx.http,
                format!(
                    ":corn::corn::corn: YOU SAID THE CORN WORD! {} :corn::corn::corn:",
                    &words.light_word
                ),
            ) {
                println!("Error sending message: {}", why)
            }
            let new_str = get_random_word(&self.light_words);
            words.light_word = new_str;
        }
        else if msg
            .content
            .to_lowercase()
            .contains(&words.dark_word.to_lowercase())
        {
            //let rnd: f32 = thread_rng().gen_range(0f32, 1f32);
            //if rnd < 0.75f32 {
                if let Err(why) = msg.channel_id.say(
                    &ctx.http,
                    format!(
                        ":corn::corn::corn: YOU SAID THE CORN WORD! {} :corn::corn::corn:",
                        &words.dark_word
                    ),
                ) {
                    println!("Error sending message: {}", why)
                }
                let new_str = get_random_word(&self.dark_words);
                words.dark_word = new_str;
            //}
        }

        
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
