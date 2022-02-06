use std::env;
use std::borrow::Cow;
pub mod data;
use data::Data as Datas;
use mammut::{Data, Mastodon, StatusBuilder, MediaBuilder};
use seahorse::{App, Command, Context, Flag, FlagType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("msr [option] [x]")
        .command(
            Command::new("status")
            .usage("msr s")
            .description("status")
            .alias("s")
            .action(s),
            )
        .command(
            Command::new("post")
            .usage("msr p {}")
            .description("post message")
            .alias("p")
            .action(p),
            )
        .command(
            Command::new("timeline")
            .usage("msr t")
            .description("timeline")
            .alias("t")
            .action(t),
            )
        .command(
            Command::new("notify")
            .usage("msr n")
            .description("notification")
            .alias("n")
            .action(n),
            )
        .command(
            Command::new("delete")
            .usage("msr d")
            .description("delete latest post")
            .alias("d")
            .action(d),
            )
        .command(c_media_upload());
    app.run(args);
}

fn token() -> Mastodon {
    let data = Datas::new().unwrap();
    let data = Data {
        base: data.base,
        token: data.token,
        client_id: data.client_id,
        client_secret: data.client_secret,
        redirect: data.redirect,
    };
    let t = Mastodon::from_data(data);
    return t;
}

fn s(_c: &Context) {
    let mastodon = token();
    let tl = mastodon.verify_credentials();
    println!("{:#?}", tl);
}

fn timeline() -> mammut::Result<()> {
    let mastodon = token();
    let length = &mastodon.get_home_timeline()?.initial_items.len();
    for n in 0..*length {
        let user = &mastodon.get_home_timeline()?.initial_items[n].account.username;
        let body = &mastodon.get_home_timeline()?.initial_items[n].content;
        println!("{} {:?}", user, body);
    }
    Ok(())
}

fn t(_c: &Context) {
    let t = timeline().unwrap();
    println!("{:#?}", t);
}

fn p(c: &Context) {
    let mastodon = token();
    let message = c.args[0].to_string();
    let status_b = StatusBuilder::new(format!("{}", message));
    let post = mastodon.new_status(status_b);
    println!("{:?}", post);
}

#[allow(unused_must_use)]
fn delete() -> mammut::Result<()> {
    let mastodon = token();
    let user = &mastodon.get_home_timeline()?.initial_items[0].account.username;
    let body = &mastodon.get_home_timeline()?.initial_items[0].content;
    let id = &mastodon.get_home_timeline()?.initial_items[0].id;
    println!("delete -> {} {:?}", user, body);
    mastodon.delete_status(id);
    Ok(())
}

fn d(_c: &Context) {
    let t = delete().unwrap();
    println!("{:#?}", t);
}

fn c_media_upload() -> Command {
    Command::new("media")
        .usage("msr media [file...]")
        .action(media)
        .alias("m")
        .description("media upload")
        .flag(
            Flag::new("text", FlagType::String)
            .description("post flag(ex. msr m ./test.png  -p text)")
            .alias("p"),
            )
}

fn media(c: &Context) {
    let mastodon = token();
    let file = c.args[0].to_string();
    if let Ok(text) = c.string_flag("text") {
        // test command
        let s = Cow::Owned(String::from(text));
        let t = mastodon.media(
            MediaBuilder::new(file.into())
            .description(s)
            .focus(200.0, 200.0)
            );
        println!("{:?}", t);
    }  else {
        let t = mastodon.media(file.into());
        let id = t.as_ref().unwrap();
        println!("{:?}", id);
        let mid = Some(vec![id.id.to_string()]);
        let status = "#media".to_string();
        println!("{:?}", mid);
        let status_b = StatusBuilder {
            status: status,
            in_reply_to_id: None,
            media_ids: mid,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        let post = mastodon.new_status(status_b);
        println!("{:?}", post);
    }
}

#[allow(unused_must_use)]
fn notify() -> mammut::Result<()> {
    let mastodon = token();
    let t = &mastodon.notifications()?.initial_items;
    println!("{:#?}", t);
    Ok(())
}

fn n(_c: &Context) {
    let t = notify().unwrap();
    println!("{:#?}", t);
}

