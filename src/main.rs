use std::env;
pub mod data;
use data::Data as Datas;
use mammut::{Data, Mastodon, StatusBuilder};
use seahorse::{App, Command, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("msr [option] [x]")
        .command(
            Command::new("status")
            .usage("msr s status")
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
            Command::new("tl")
            .usage("msr t")
            .description("timeline")
            .alias("t")
            .action(t),
            )
        .command(
            Command::new("delete")
            .usage("msr d")
            .description("delete latest post")
            .alias("d")
            .action(d),
            );
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
