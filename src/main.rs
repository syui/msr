pub mod data;
use std::env;
use std::borrow::Cow;
use std::fs;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use data::Data as Datas;
use data::Datam as Datams;
use data::Set as Sets;
use data::Deep as Deeps;
use mammut::{Data, Mastodon, StatusBuilder, MediaBuilder};
use seahorse::{App, Command, Context, Flag, FlagType};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};
// misskey
use futures::stream::TryStreamExt;
use anyhow::Result;
use misskey::prelude::*;
use misskey::{WebSocketClient, HttpClient};
// reqwest
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    mid: String,
}

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
            .action(s)
            .flag(
                Flag::new("user", FlagType::String)
                .description("user flag(ex: $ msr s -u user)")
                .alias("u"),
                )
            .flag(
                Flag::new("id", FlagType::String)
                .description("id flag(ex: $ msr s -i user)")
                .alias("i"),
               )
            .flag(
                Flag::new("timeline", FlagType::Bool)
                .description("Timeline flag")
                .alias("t"),
                )
            .flag(
                Flag::new("all", FlagType::Bool)
                .description("All flag")
                .alias("a"),
                )
            )
        .command(
            Command::new("post")
            .usage("msr p {}")
            .description("post message, ex: $ msr p $text")
            .alias("p")
            .action(p)
            .flag(
                Flag::new("lang", FlagType::String)
                .description("Lang flag")
                .alias("l"),
                )
            )
        .command(
            Command::new("translate")
            .usage("msr tt {}")
            .description("translate message, ex: $ msr tt $text -l en")
            .alias("tt")
            .action(tt)
            .flag(
                Flag::new("lang", FlagType::String)
                .description("Lang flag")
                .alias("l"),
                )
            .flag(
                Flag::new("api", FlagType::String)
                .description("set api deepl")
                .alias("a"),
                )
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
            .description("notification, ex: $ msr n --clear")
            .alias("n")
            .action(n)
            .flag(
                Flag::new("clear", FlagType::Bool)
                .description("Clear flag")
                .alias("c"),
                )
            )
        .command(
            Command::new("bot")
            .usage("msr bot")
            .description("bot, ex: $ msr n --clear")
            .action(bot)
            )
        .command(
            Command::new("notifylatest")
            .usage("msr nl")
            .description("notification-latest, ex: $msr nl -o id")
            .alias("nl")
            .action(nl)
            .flag(
                Flag::new("text", FlagType::String)
                .description("post flag(ex: $ msr nl -o $text)")
                .alias("o"),
                )
            )
        .command(
            Command::new("notifysecond")
            .usage("msr nls")
            .description("notification, ex: $msr nls")
            .alias("nls")
            .action(nls)
            .flag(
                Flag::new("text", FlagType::String)
                .description("post flag(ex: $ msr nls -o $text)")
                .alias("o"),
                )
            )
        .command(
            Command::new("mention")
            .usage("msr mention {}")
            .description("mention, ex: $ msr mm $id -p $text")
            .alias("mm")
            .action(mention)
            .flag(
                Flag::new("text", FlagType::String)
                .description("post flag(ex: $ msr mm $id -p $text)")
                .alias("p"),
                )
            .flag(
                Flag::new("set", FlagType::String)
                .description("mention id set post flag(ex: $ msr mm $text -s $id)")
                .alias("s"),
                )
            )
        .command(
            Command::new("delete")
            .usage("msr d")
            .description("delete latest post")
            .alias("d")
            .action(d)
            .flag(
                Flag::new("id", FlagType::String)
                .description("Delete id flag")
                .alias("i"),
                )
            )
        .command(
            Command::new("icon")
            .usage("msr i")
            .description("timeline view avator")
            .alias("i")
            .action(icon_t),
            )
        .command(
            Command::new("follow")
            .usage("msr f {}")
            .description("follow, ex: $ msr f @ai@syui.cf")
            .alias("f")
            .action(f)
            .flag(
                Flag::new("delete", FlagType::Bool)
                .description("Delete flag")
                .alias("d"),
                )
            .flag(
                Flag::new("list", FlagType::Bool)
                .description("followers list flag")
                .alias("l"),
                )
            .flag(
                Flag::new("ll", FlagType::Bool)
                .description("user followers list flag, $ msr f @syui@syui.cf -ll")
                .alias("ll"),
                )
            )
        .command(
            Command::new("reblog")
            .usage("msr r {}")
            .description("reblog, ex: $ msr r $id")
            .alias("r")
            .action(r)
            .flag(
                Flag::new("delete", FlagType::Bool)
                .description("Delete flag")
                .alias("d"),
                )
            )
        .command(
            Command::new("fav")
            .usage("msr fa {}")
            .description("fav, ex: $ msr fa $id")
            .alias("fa")
            .action(fa)
            .flag(
                Flag::new("delete", FlagType::Bool)
                .description("Delete flag")
                .alias("d"),
                )
            )
        .command(
            Command::new("accont")
            .usage("msr a {}")
            .description("account change, ex : $ msr a ~/test.toml, $ msr a -d(setting.toml)")
            .alias("a")
            .action(a),
            )
        .command(c_media_upload())

        // misskey command
        .command(
            Command::new("misskey")
            .usage("msr misskey")
            .description("post message, ex: $ msr misky -t")
            .alias("misky")
            .action(misskey_t)
            .flag(
                Flag::new("post", FlagType::String)
                .description("post flag(ex: $ msr misky -p $text)")
                .alias("p"),
                )
            .flag(
                Flag::new("timeline", FlagType::Bool)
                .description("timeline flag(ex: $ msr misky -t)")
                .alias("t"),
                )
            )

        // other command
        .command(
            Command::new("char")
            .usage("msr char {}")
            .description("char change, ex : $ msr char $text")
            .alias("char")
            .action(char_c)
            .flag(
                Flag::new("limit", FlagType::Int)
                .description("limit flag")
                .alias("l"),
                )
            )
        // other command
        .command(
            Command::new("char-next")
            .usage("msr char-next {}")
            .description("char, ex : $ msr cn $text -mm $mid")
            .alias("cn")
            .action(char_cn)
            .flag(
                Flag::new("mid", FlagType::String)
                .description("mid flag")
                .alias("mm"),
                )
            .flag(
                Flag::new("limit", FlagType::String)
                .description("limit flag")
                .alias("l"),
                )
            )
        .command(
            Command::new("char-line")
            .usage("msr char-line {}")
            .description("char line, ex : $ msr cl $text")
            .alias("cl")
            .action(char_cl)
            .flag(
                Flag::new("mid", FlagType::String)
                .description("mid flag")
                .alias("mm"),
                )
            .flag(
                Flag::new("limit", FlagType::String)
                .description("limit flag")
                .alias("l"),
                )
            )
        ;
    app.run(args);
}

#[derive(Serialize, Deserialize)]
struct Address {
    user : String,
    id : String,
    mid : String,
    url : String,
    date : String,
    body : String,
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

#[tokio::main]
async fn deepl(message: String,lang: String) -> Result<()> {
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
    let l = shellexpand::tilde("~") + "/.config/msr/deepl.txt";
    let l = l.to_string();
    let mut l = fs::File::create(l).unwrap();
    if o != "" {
        l.write_all(&o.as_bytes()).unwrap();
    }
    println!("{}", o);
    Ok(())
}

fn timeline() -> mammut::Result<()> {
    let mastodon = token();
    let tmp = &mastodon.get_home_timeline()?.initial_items;
    let length = &tmp.len();
    for n in 0..*length {
        let nn = &tmp[n];
        let id = &nn.id;
        let user = &nn.account.username;
        let body = &nn.content;
        let reblog = &nn.reblog;
        if body.is_empty() == true {
            let ruser = &reblog.as_ref().unwrap().uri;
            let rbody = &reblog.as_ref().unwrap().content;
            println!("re:{} {:?} {:?} {:?}", user, ruser, rbody, id);
        } else {
            println!("{} {:?} {:?}", user, body, id);
        }
    }
    Ok(())
}

fn t(_c: &Context) {
    timeline().unwrap();
}

#[allow(unused_must_use)]
fn p(c: &Context) {
    let mastodon = token();
    let message = c.args[0].to_string();
    let m = c.args[0].to_string();
    if let Ok(lang) = c.string_flag("lang") {
        deepl(m,lang.to_string());
        let l = shellexpand::tilde("~") + "/.config/msr/deepl.txt";
        let l = l.to_string();
        let o = fs::read_to_string(&l).expect("could not read file");
        let status_b = StatusBuilder::new(format!("{}", o.to_string()));
        let post = mastodon.new_status(status_b);
        println!("{:?}", post);
    } else {
        let status_b = StatusBuilder::new(format!("{}", message));
        let post = mastodon.new_status(status_b);
        println!("{:?}", post);
    }
}

#[allow(unused_must_use)]
fn tt(c: &Context) {
    if let Ok(api) = c.string_flag("api") {
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
    let m = c.args[0].to_string();
    if let Ok(lang) = c.string_flag("lang") {
        deepl(m,lang.to_string());
    } else {
        let lang = "ja";
        deepl(m,lang.to_string());
    }
}

fn msr_set_user(c: &Context) -> io::Result<()> {
    let path = "/.config/msr/";
    let file = path.to_string() + &"set.toml";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    println!("{}", f);
    let check = Path::new(&f).exists();
    if check == true {
        let o = fs::read_to_string(&f)?;
        println!("read {}", o);           
    }
    let mut f = fs::File::create(f)?;

    if let Ok(set) = c.string_flag("set") {
        if set.is_empty() == false {
            let set = &*set.to_string();
            let setting = Setting {
                mid: set.into(),
            };
            let toml = toml::to_string(&setting).unwrap();
            write!(f, "{}", toml)?;
            f.flush()?;
            println!("\n#TOML:\n{}", toml);
        }
    }
    Ok(())
}

#[allow(unused_must_use)]
fn mention_post(mid: String, s: String) -> mammut::Result<()> {
    let mastodon = token();
    let status_b = StatusBuilder {
        status: s,
        in_reply_to_id: Some(mid),
        media_ids: None,
        sensitive: None,
        spoiler_text: None,
        visibility: None,
    };
    let post = mastodon.new_status(status_b);
    println!("{:#?}",post);
    Ok(())
}

#[allow(unused_must_use)]
fn mention(c: &Context) {
    if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        let mid = c.args[0].to_string();
        mention_post(mid, status.to_string());
    }
    if let Ok(set) = c.string_flag("set") {
        let status = c.args[0].to_string();
        mention_post(set, status);
        msr_set_user(c);
    } else {
        let path = "/.config/msr/";
        let file = path.to_string() + &"set.toml";
        let mut f = shellexpand::tilde("~").to_string();
        f.push_str(&file);
        println!("{}", f);
        let data = Sets::new().unwrap();
        let status = c.args[0].to_string();
        mention_post(data.mid, status);
    }
}

#[allow(unused_must_use)]
fn delete(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    if let Ok(id) = c.string_flag("id") {
        mastodon.delete_status(&id);
    } else {
        let n = &mastodon.get_home_timeline()?.initial_items[0];
        let user = &n.account.username;
        let body = &n.content;
        let id = &n.id;
        println!("delete -> {} {:?}", user, body);
        mastodon.delete_status(id);
    }
    Ok(())
}

fn d(c: &Context) {
    delete(c).unwrap();
}

fn c_media_upload() -> Command {
    Command::new("media")
        .usage("msr media [file...]")
        .action(media)
        .alias("m")
        .description("media upload, ex: $ msr m ./test.png -p text")
        .flag(
            Flag::new("text", FlagType::String)
            .description("post flag(ex: $ msr m ./test.png  -p text)")
            .alias("p"),
            )
        .flag(
            Flag::new("uri", FlagType::Bool)
            .description("Uri flag")
            .alias("u"),
            )
        .flag(
            Flag::new("rid", FlagType::String)
            .description("Mention flag")
            .alias("r"),
            )
}

fn media(c: &Context) {
    let mastodon = token();
    let file = c.args[0].to_string();
    let t = if let Ok(text) = c.string_flag("text") {
        let t = mastodon.media(
            MediaBuilder::new(file.into())
            .description(Cow::Owned(String::from(text)))
            //.focus(200.0, 200.0)
            );
        t
    } else {
        let t = mastodon.media(file.into());
        t
    };
    let id = t.as_ref().unwrap();
    let mid = Some(vec![id.id.to_string()]);
    let status_b = if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        let status_b = StatusBuilder {
            status: status.to_string(),
            in_reply_to_id: None,
            media_ids: mid,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        status_b
    } else if let Ok(rid) = c.string_flag("rid") {
        let status = "#media".to_string();
        let rid = &*rid.to_string();
        let status_b = StatusBuilder {
            status: status,
            in_reply_to_id: Some(rid.to_string()),
            media_ids: mid,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        status_b
    } else {
        let status = "#media".to_string();
        let status_b = StatusBuilder {
            status: status,
            in_reply_to_id: None,
            media_ids: mid,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        status_b
    };
    let post = mastodon.new_status(status_b);
    if c.bool_flag("uri") {
        let body = post.unwrap().uri;
        println!("{:#?}", body);
    } else {
        println!("{:?}", post);
    }
}

#[allow(unused_must_use)]
fn notify(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let nn = &mastodon.notifications()?.initial_items;
    println!("{:#?}", nn);
    if c.bool_flag("clear") {
        println!("{:#?}", "clear_notifications");
        mastodon.clear_notifications();
    } 
    Ok(())
}

fn n(c: &Context) {
    notify(c).unwrap();
}

fn notifylatest(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let nn = &mastodon.notifications()?.initial_items;
    if nn.len() == 0 {
        println!("{:#?}", nn);
        return Ok(());
    }
    let n = &nn[0];
    let date = &n.created_at;
    let ntype = &n.notification_type;
    let user = &n.account.username;
    let id = &n.id;
    let url = &n.account.url;
    let b = &n.status;
    let body = &b.as_ref().unwrap().content;
    let mid = &b.as_ref().unwrap().id;
    let address = Address {
        user : user.to_owned(),
        id : id.to_owned(),
        mid : mid.to_owned(),
        url : url.to_owned(),
        date : date.to_owned().to_string(),
        body : body.to_owned(),
    };
    let j = serde_json::to_string(&address)?;
    if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        if b.is_none() {
            let opt: Option<i32> = None;
            println!("{:?}", opt);
        } else {
            match &*status {
                "id" => println!("{}", id),
                "mid" => println!("{}", mid),
                "user" => println!("{}", user),
                "date" => println!("{}", date),
                "body" => println!("{}", body),
                "url" => println!("{}", url),
                "type" => println!("{:#?}", ntype),
                "status" => println!("{:#?}", b),
                _ => println!("not matched(id, mid, user, date, body, url, type, status)"),
            }
        }
    } else {
        if b.is_none() {
            let opt: Option<i32> = None;
            println!("{:?}", opt);
        } else {
            println!("{}", j);
        }
    }
    Ok(())
}

fn nl(c: &Context) {
    notifylatest(c).unwrap();
}

fn bot_run(_c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let nn = &mastodon.notifications()?.initial_items;
    if nn.len() == 0 {
        println!("{:#?}", nn);
        return Ok(());
    }
    let length = &nn.len();
    for n in 0..*length {
        let tmp = &nn[n];
        let date = &tmp.created_at;
        let ntype = &tmp.notification_type;
        let user = &tmp.account.username;
        let id = &tmp.id;
        let url = &tmp.account.url;
        let b = &tmp.status;
        if b.is_none() {
            continue;
        }
        let body = &b.as_ref().unwrap().content;
        let mid = &b.as_ref().unwrap().id;
        let address = Address {
            user : user.to_owned(),
            id : id.to_owned(),
            mid : mid.to_owned(),
            url : url.to_owned(),
            date : date.to_owned().to_string(),
            body : body.to_owned(),
        };
        let j = serde_json::to_string(&address)?;
        println!("{}", j);
    }
    Ok(())
}

fn bot(c: &Context) {
    bot_run(c).unwrap();
}

fn notifysecond(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let nn = &mastodon.notifications()?.initial_items;
    if nn.len() == 0 {
        println!("{:#?}", nn);
        return Ok(());
    }
    let length = &nn.len();
    for n in 0..*length {
        let tmp = &nn[n];
        let date = &tmp.created_at;
        let ntype = &tmp.notification_type;
        let user = &tmp.account.username;
        let id = &tmp.id;
        let url = &tmp.account.url;
        let b = &tmp.status;
        if b.is_none() {
            continue;
        }
        let body = &b.as_ref().unwrap().content;
        let mid = &b.as_ref().unwrap().id;
        let address = Address {
           user : user.to_owned(),
           id : id.to_owned(),
           mid : mid.to_owned(),
           url : url.to_owned(),
           date : date.to_owned().to_string(),
           body : body.to_owned(),
        };
        let j = serde_json::to_string(&address)?;
        if let Ok(text) = c.string_flag("text") {
            let status = &*text.to_string();
            match &*status {
                "id" => println!("{}", id),
                "mid" => println!("{}", mid),
                "user" => println!("{}", user),
                "date" => println!("{}", date),
                "body" => println!("{}", body),
                "url" => println!("{}", url),
                "type" => println!("{:#?}", ntype),
                "status" => println!("{:#?}", b),
                _ => println!("not matched(id, mid, user, date, body, url, type, status)"),
            }
        } else {
            if n == 0 {
                println!("{}", "[");
            }
            println!("{}", j);
            if n != nn.len() - 1{
                println!("{}", ",");
            }
            if n == nn.len() - 1{
                println!("{}", "]");
            }
        }
    }
    Ok(())
}

fn nls(c: &Context) {
    notifysecond(c).unwrap();
}

fn get_domain_zsh() {
    let data = Datas::new().unwrap();
    let data = Data {
        base: data.base,
        token: data.token,
        client_id: data.client_id,
        client_secret: data.client_secret,
        redirect: data.redirect,
    };
    let e = "export MASTODON_BASE=".to_owned() + &data.base.to_string() + "\n";
    let e = e.to_string();
    let f = shellexpand::tilde("~") + "/.config/msr/msr.zsh";
    let f = f.to_string();
    let r = shellexpand::tilde("~") + "/.config/msr/msr.zsh";
    let r = r.to_string();
    fs::remove_file(r).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let mut f = fs::File::create(f).unwrap();
    f.write_all(e.as_bytes()).unwrap();
    //let mastodon = token();
    //let account = mastodon.verify_credentials();
    //let user = account.unwrap().username;
    //let u = "export MASTODON_USER=".to_owned() + &user;
    //f.write_all(u.as_bytes()).unwrap();
    //let src = "exec $SHELL && . ~/.zshrc";
    //println!("{}", src);
}

#[allow(unused_must_use)]
fn a(c: &Context)  {
    let i = c.args[0].to_string();
    let o = shellexpand::tilde("~") + "/.config/msr/config.toml";
    let o = o.to_string();
    if &i == "-d" {
        let i = shellexpand::tilde("~") + "/.config/msr/setting.toml";
        let i = i.to_string();
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    } else if &i == "-s" {
        let i = shellexpand::tilde("~") + "/.config/msr/social.toml";
        let i = i.to_string();
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    } else {
        println!("{:#?} -> {:#?}", i, o);
        fs::copy(i, o);
    }
    get_domain_zsh();
}

fn icon(filef: String) {
    use std::process::Command;
    let path = "/.config/msr/icon/";
    let file = path.to_string() + &filef;
    //let file = path.to_string() + &user + &"-min.png";
    let mut f = shellexpand::tilde("~").to_string();
    f.push_str(&file);
    match os_type::current_platform().os_type {
        os_type::OSType::OSX => {
            // which imgcat
            // curl -sL https://iterm2.com/utilities/imgcat
            Command::new("imgcat").arg(f).spawn().expect("imgcat");
        }
        os_type::OSType::Arch => {
            // pacman -S libsixel
            Command::new("img2sixel").arg(f).arg("-h 25").spawn().expect("sixel");
        }
        os_type::OSType::Ubuntu => {
            // apt-get install -y libsixel-bin
            Command::new("img2sixel").arg(f).arg("-h 25").spawn().expect("sixel");
        }
        _ => {
            if cfg!(target_os = "windows") {
                Command::new("img2sixel").arg(f).arg("-h 25").spawn().expect("sixel");
            };
        }
    }
}

fn icon_timeline() -> mammut::Result<()> {
    let mastodon = token();
    let tmp = &mastodon.get_home_timeline()?.initial_items;
    let length = &tmp.len();
    for n in 0..*length {
        let nn = &tmp[n];
        let avator = &nn.account.avatar_static;
        let user = &nn.account.username;
        let body = &nn.content;
        let reblog = &nn.reblog;
        let path = "/.config/msr/icon/";
        let fend = Path::new(&avator).extension().unwrap().to_str().unwrap();
        let file = path.to_string() + &user + &"." + &fend;
        let filef = user.to_string() + &"." + &fend;
        let mut p = shellexpand::tilde("~").to_string();
        let mut f = shellexpand::tilde("~").to_string();
        let mut i = shellexpand::tilde("~").to_string();
        p.push_str(&path);
        f.push_str(&file);
        i.push_str(&file);
        match fs::create_dir_all(p) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
        let check = Path::new(&f).exists();
        if check == false {
            let mut dst = Vec::new();
            let mut easy = Easy::new();
            easy.url(avator).unwrap();
            let _redirect = easy.follow_location(true);
            {
                let mut transfer = easy.transfer();
                transfer.write_function(|data| {
                    dst.extend_from_slice(data);
                    Ok(data.len())
                }).unwrap();
                transfer.perform().unwrap();
            }
            {
                let mut file = File::create(f)?;
                file.write_all(dst.as_slice())?;
            }
        }
        icon(filef.to_string());
        if body.is_empty() == true {
            let ruser = &reblog.as_ref().unwrap().uri;
            let rbody = &reblog.as_ref().unwrap().content;
            println!("re:{} {:?} {:?}", user, ruser, rbody);
        } else {
            println!("{} {:?}", user, body);
        }
        //let img = image::open(i).unwrap();
        //let resized = image::imageops::resize(&img, 25, 25, image::imageops::Lanczos3);
        //let check = Path::new(&m).exists();
        //if check == false {
        //    resized.save(m).unwrap();
        //}
    }
    Ok(())
}

fn icon_t(_c: &Context) {
    icon_timeline().unwrap();
}

#[allow(unused_must_use)]
fn follow(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    if c.bool_flag("list") {
        let status = mastodon.verify_credentials().unwrap();
        let id = status.id;
        let status = mastodon.followers(&id)?.initial_items;
        let length = &status.len();
        for n in 0..*length {
            let nn = &status[n];
            let acct = &nn.acct;
            println!("{}", acct);
        }
    } else {
        let id = c.args[0].to_string();
        let status = mastodon.search_accounts(&id, None, false)?.initial_items;
        let nn = &status[0];
        let acct = &nn.acct;
        let id = &nn.id;
        if c.bool_flag("delete") {
            println!("{:#?}", "unfollow");
            mastodon.unfollow(&id);
        } else if c.bool_flag("ll") {
            let status = mastodon.followers(&id)?.initial_items;
            let length = &status.len();
            for n in 0..*length {
                let nn = &status[n];
                let acct = &nn.acct;
                println!("{}", acct);
            }
        } else {
            mastodon.follow(&id);
            println!("follow : {:?} {:?}", id, acct);
        }
    }
    Ok(())
}

fn f(c: &Context) {
    follow(c).unwrap();
}

#[allow(unused_must_use)]
fn status(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    if let Ok(user) = c.string_flag("user") {
        let status = mastodon.search_accounts(&user, None, false)?.initial_items;
        println!("{:#?}", status);
    } else if let Ok(id) = c.string_flag("id") {
        let status = &mastodon.search_accounts(&id, None, false)?.initial_items;
        let length = &status.len();
        for n in 0..*length {
            let nn = &status[n];
            let acct = &nn.acct;
            let id = &nn.id;
            if c.bool_flag("timeline") {
                println!("{:#?}", acct);
                let tl = &mastodon.statuses(&id, None)?.initial_items;
                if c.bool_flag("all") {
                    println!("{:#?}", tl);
                } else {
                    let length_tl = &tl.len();
                    for nnn in 0..*length_tl {
                        let body = &tl[nnn].content;
                        let mid = &tl[nnn].id;
                        if body.is_empty() == false {
                            println!("{:#?} {:#?}", mid, body);
                        } else {
                            let reblog = &tl[nnn].reblog.as_ref().unwrap().content;
                            println!("{:#?} {:#?}", mid, reblog);
                        }
                    }
                }
            } else {
                println!("{:#?} {:#?}", acct, id);
            }
        }
    } else {
        let status = mastodon.verify_credentials();
        println!("{:#?}", status);
    }
    Ok(())
}

fn s(c: &Context) {
    status(c).unwrap();
}

#[allow(unused_must_use)]
fn reblog(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let id = c.args[0].to_string();
    if c.bool_flag("delete") {
        println!("{:#?}", "unreblog");
        mastodon.unreblog(&id);
    } else {
        println!("{:#?}", "reblog");
        mastodon.reblog(&id);
    }
    Ok(())
}

fn r(c: &Context) {
    reblog(c).unwrap();
}

#[allow(unused_must_use)]
fn fav(c: &Context) -> mammut::Result<()> {
    let mastodon = token();
    let id = c.args[0].to_string();
    if c.bool_flag("delete") {
        println!("{:#?}", "unfav");
        mastodon.unfavourite(&id);
    } else {
        println!("{:#?}", "fav");
        mastodon.favourite(&id);
    }
    Ok(())
}

fn fa(c: &Context) {
    fav(c).unwrap();
}

// misskey
#[tokio::main]
async fn misskey_post(c: &Context) -> anyhow::Result<()> {
    let opt = Datams::new().unwrap();
    if let Ok(post) = c.string_flag("post") {
        let i = &*post.to_string();
        let client = HttpClient::builder(opt.misskey_api)
            .token(opt.misskey_token)
            .build()?;
        client.create_note(i).await?;
    }
    Ok(())
}

#[tokio::main]
async fn misskey_timeline() -> Result<()> {
    let opt = Datams::new().unwrap();
    println!("{:?}", opt.misskey_stream);
    let client = WebSocketClient::builder(opt.misskey_stream)
        .token(opt.misskey_token)
        .connect()
        .await?;
    let mut stream = client.local_timeline().await?;
    while let Some(note) = stream.try_next().await? {
        println!(
            "<@{}> {}",
            note.user.username,
            note.text.unwrap_or_default()
            );
    }
    Ok(())
}

fn misskey_t(c: &Context) {
    if c.bool_flag("timeline") {
        misskey_timeline().unwrap();
    }
    if let Ok(_post) = c.string_flag("post") {
        misskey_post(c).unwrap();
    }
}

fn char_l(c: &Context) -> i32 {
    if let Ok(limit) = c.int_flag("limit") {
        let l: i32 = limit.try_into().unwrap();
        let l = l - 1;
        return l.try_into().unwrap();
    } else {
        let l = 490;
        return l;
    }
}

fn char_c(c: &Context) {
    let i = c.args[0].to_string();
    let mut s = String::new();
    let l = char_l(c);
    for ii in i.chars().enumerate() {
        match ii.0 {
            n if n > l.try_into().unwrap() => {break}
            _ => {s.push(ii.1)}
        }
    }
    println!("{}",s);
}

#[allow(unused_must_use)]
fn char_cn(c: &Context) {
    let i = c.args[0].to_string();
    let mut s = String::new();
    let mut sa = String::new();
    let mut sb = String::new();
    let mut sc = String::new();
    let mut sd = String::new();
    for ii in i.chars().enumerate() {
        match ii.0 {
            0 ..= 450 => {
                sa.push(ii.1);
            }
            451..=950 => {
                sb.push(ii.1);
            }
            951..=1450 => {
                sc.push(ii.1);
            }
            1451..=1950 => {
                sd.push(ii.1);
            }
            _ => {s.push(ii.1)}
        }
    }
    if let Ok(mid) = c.string_flag("mid") {
        if sa.is_empty() == false {
            println!("{}",sa);
            mention_post(mid,sa);
        }
    }
    if let Ok(mid) = c.string_flag("mid") {
        if sb.is_empty() == false {
            println!("{}",sb);
            mention_post(mid,sb);
        }
    }
    if let Ok(mid) = c.string_flag("mid") {
        if sc.is_empty() == false {
            println!("{}",sc);
            mention_post(mid,sc);
        }
    }
    if let Ok(mid) = c.string_flag("mid") {
        if sd.is_empty() == false {
            println!("{}",sd);
            mention_post(mid,sd);
        }
    }
    //mention_post(mid.to_string(),s);
    //println!("{}",s);
}

#[allow(unused_must_use)]
fn char_cl(c: &Context) {
    let i = c.args[0].to_string();
    let mut s = String::new();
    let ll = 450;
    let mut counter = 1;
    let max = i.chars().count();
    let nn = max / ll;
    loop {
        if counter == nn {
            break;
        }
        for ii in i.chars().enumerate() {
            match ii.0 {
                n if n == max => {
                    if let Ok(mid) = c.string_flag("mid") {
                        if s.is_empty() == false {
                            println!("{}",s);
                            let t = s.pop();
                            mention_post(mid,t.expect("REASON").to_string());
                        }
                    }
                }
                n if n == ll * counter => {
                    if let Ok(mid) = c.string_flag("mid") {
                        if s.is_empty() == false {
                            println!("{}",s);
                            let t = s.pop();
                            mention_post(mid,t.expect("REASON").to_string());
                        }
                    }
                    let _s = String::new();
                }
                _ => {s.push(ii.1)}
            }
        }
        counter += 1;
    }
}

