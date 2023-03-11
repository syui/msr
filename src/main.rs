pub mod data;
use std::env;
use std::fs;
use std::io::prelude::*;
use data::Open as Opens;
use seahorse::{App, Command, Context, Flag, FlagType};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct OpenData {
    choices: Vec<Choices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choices {
    text: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("chatgpt-rs [option] [x]")
        .command(
            Command::new("chatgpt")
            .usage("chatgpt tt {}")
            .description("chatgpt message, ex: $ chatgpt-rs tt $text -l en")
            .alias("tt")
            .action(openai_post)
            .flag(
                Flag::new("model", FlagType::String)
                .description("model flag")
                .alias("m"),
                )
            )
        .command(
            Command::new("api")
            .usage("chatgpt-rs a {}")
            .description("api change, ex : $ msr a $api")
            .alias("a")
            .action(openai_api),
            )
        ;
    app.run(args);
}

#[tokio::main]
async fn openai(prompt: String, model: String) -> reqwest::Result<()> {
    let data = Opens::new().unwrap();
    let data = Opens {
        api: data.api,
    };
    let temperature = 0.7;
    let max_tokens = 250;
    let top_p = 1;
    let frequency_penalty = 0;
    let presence_penalty = 0;
    let stop = "[\"###\"]";

    let post = Some(json!({
        "prompt": &prompt.to_string(),
        "model": &model.to_string(),
        "temperature": temperature,
        "max_tokens": max_tokens,
        "top_p": top_p,
        "frequency_penalty": frequency_penalty,
        "presence_penalty": presence_penalty,
        "stop": stop,
    }));
        
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/completions")
        .header("Authorization", "Bearer ".to_owned() + &data.api)
        .json(&post)
        .send()
        .await?
        .text()
        .await?;
    let p: OpenData = serde_json::from_str(&res).unwrap();
    let o = &p.choices[0].text;
    let o: String = o.chars().filter(|c| !c.is_whitespace()).collect();
    println!("{}", o);
    Ok(())
}

#[allow(unused_must_use)]
fn openai_post(c: &Context) {
    let m = c.args[0].to_string();
    if let Ok(model) = c.string_flag("model") {
        openai(m,model.to_string());
    } else {
        let model = "text-davinci-003";
        openai(m,model.to_string());
    }
}

#[allow(unused_must_use)]
fn openai_api(c: &Context) {
    let api = c.args[0].to_string();
    let o = "api='".to_owned() + &api.to_string() + &"'".to_owned();
    let o = o.to_string();
    let l = shellexpand::tilde("~") + "/.config/msr/openai.toml";
    let l = l.to_string();
    let mut l = fs::File::create(l).unwrap();
    if o != "" {
        l.write_all(&o.as_bytes()).unwrap();
    }
    println!("{:#?}", l);
}

