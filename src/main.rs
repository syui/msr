pub mod data;
use std::env;
use std::fs;
use std::io::prelude::*;
use data::Deep as Deeps;
use seahorse::{App, Command, Context, Flag, FlagType};
use serde::{Deserialize, Serialize};
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct DeepData {
    translations: Vec<Translation>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Translation {
    text: String,
    detected_source_language : String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("trans-rs [option] [x]")
        .command(
            Command::new("translate")
            .usage("trans-rs tt {}")
            .description("translate message, ex: $ trans-rs tt $text -l en")
            .alias("tt")
            .action(tt)
            .flag(
                Flag::new("lang", FlagType::String)
                .description("Lang flag")
                .alias("l"),
                )
            )
        .command(
            Command::new("api")
            .usage("trans-rs a {}")
            .description("api change, ex : $ msr a $api")
            .alias("a")
            .action(a),
            )
        ;
    app.run(args);
}

#[tokio::main]
async fn deepl(message: String,lang: String) -> reqwest::Result<()> {
    let data = Deeps::new().unwrap();
    let data = Deeps {
        api: data.api,
    };
    let api = "DeepL-Auth-Key ".to_owned() + &data.api;
    let mut params = HashMap::new();
    params.insert("text", &message);
    params.insert("target_lang", &lang);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api-free.deepl.com/v2/translate")
        .header(AUTHORIZATION, api)
        .header(CONTENT_TYPE, "json")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    let p: DeepData = serde_json::from_str(&res).unwrap();
    let o = &p.translations[0].text;
    //println!("{}", res);
    println!("{}", o);
    Ok(())
}

#[allow(unused_must_use)]
fn tt(c: &Context) {
    let m = c.args[0].to_string();
    if let Ok(lang) = c.string_flag("lang") {
        deepl(m,lang.to_string());
    } else {
        let lang = "ja";
        deepl(m,lang.to_string());
    }
}

#[allow(unused_must_use)]
fn a(c: &Context) {
    let api = c.args[0].to_string();
    let o = "api='".to_owned() + &api.to_string() + &"'".to_owned();
    let o = o.to_string();
    let l = shellexpand::tilde("~") + "/.config/msr/deepl.toml";
    let l = l.to_string();
    let mut l = fs::File::create(l).unwrap();
    if o != "" {
        l.write_all(&o.as_bytes()).unwrap();
    }
    println!("{:#?}", l);
}
