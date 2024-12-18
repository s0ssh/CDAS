use rocket::http::Status;
use std::fs;
use std::collections::HashMap;
use rand::prelude::IteratorRandom;
use std::io::prelude::*;
use std::io::Write;
use std::fs::File;

const STATE_SIZE: u16 = 3;
const MAX_WORDS_CAP: u16 = 1000;
const MIN_WORDS: u16 = 0;

#[get("/wisdom?<max_words>")]
pub async fn get_wisdom(max_words: Option<u16>) -> (Status, String) {
    let mut max_words = max_words.unwrap_or(100);
    if max_words > MAX_WORDS_CAP {
        max_words = MAX_WORDS_CAP;
    }

    match File::open("data/bible.model") {
        Ok(mut f) => {
            let mut model_raw: Vec<u8> = Vec::new();
            f.read_to_end(&mut model_raw).unwrap();
            let model: HashMap<String, Vec<&str>> = bincode::deserialize(&model_raw).unwrap();

            (Status::Ok, gen_markov_text(&model, STATE_SIZE.into(), MIN_WORDS.into(), max_words.into()))
        }, 
        Err(_) => {
            let bible = match fs::read_to_string("data/bible.txt") {
                Ok(b) => b,
                Err(_) => return (Status::InternalServerError, String::from(""))
            };
            let model_enc: Vec<u8> = bincode::serialize(&gen_markov_model(&bible, STATE_SIZE.into())).unwrap();
            
            let mut model_file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("data/bible.model")
                .unwrap();

            model_file.write_all(&model_enc).unwrap();
            (Status::ServiceUnavailable, String::from("WAIT A MINUTE AND TRY AGAIN"))
        }
    }
}

fn gen_markov_model(text: &str, state_size: usize) -> HashMap<String, Vec<&str>> {
    let text = text.split_whitespace().collect::<Vec<_>>();
    let mut model: HashMap<String, Vec<&str>> = HashMap::new();
    
    for i in state_size..text.len() {
        let word = text[i];
        let prev_words: String = text[i - state_size..i].join(" ");
        if model.contains_key(&prev_words) {
            match model.get_mut(&prev_words) {
                Some(v) => v.push(word),
                None => {}
            };
        } else {
            model.insert(prev_words, vec![word]);
        }
    }

    model
}

fn gen_markov_text(model: &HashMap<String, Vec<&str>>, state_size: usize, min_words: usize, max_words: usize) -> String {
    let mut msg_raw = Vec::new();

    msg_raw.append(&mut gen_markov_starter(&model));

    let mut i: usize = state_size;
    loop {
        let key: String = msg_raw[i - state_size..i].join(" ");
        if !model.contains_key(&key) {
            msg_raw.append(&mut gen_markov_starter(&model));
            i += 1;
            continue;
        }

        let next_word = model.get(&key).unwrap().into_iter().choose(&mut rand::thread_rng()).unwrap();
        msg_raw.push(next_word.to_string());

        i += 1;

        if i > min_words && i >= max_words - 1 {
            break;
        }
    }


    msg_raw.join(" ")
}

fn gen_markov_starter(model: &HashMap<String, Vec<&str>>) -> Vec<String> {
    let mut starter = Vec::new();

    for k in model.keys() {
        // TODO: may be a better way to short-circuit the is_uppercase than hardcoding a lowercase
        // char
        if k.chars().nth(0).unwrap_or('a').is_uppercase() {
            starter.append(&mut k.split(' ').map(|s| s.to_string()).collect::<Vec<String>>());
            break;
        }
    }

    starter
}
