use std::env;
use std::borrow::Cow;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
pub mod data;
use data::Data as Datas;
use mammut::{Data, Mastodon, StatusBuilder, MediaBuilder};
use seahorse::{App, Command, Context, Flag, FlagType};
use curl::easy::Easy;

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
            .action(n)
            .flag(
                Flag::new("clear", FlagType::Bool)
                .description("Clear flag")
                .alias("c"),
                )
            )
        .command(
            Command::new("notifylatest")
            .usage("msr nl")
            .description("notification-latest")
            .alias("nl")
            .action(nl)
            .flag(
                Flag::new("text", FlagType::String)
                .description("post flag(ex: $ msr nl -o text)")
                .alias("o"),
                )
            )
        .command(
            Command::new("mention")
            .usage("msr mention {}")
            .description("mention")
            .alias("mm")
            .action(mention)
            .flag(
                Flag::new("text", FlagType::String)
                .description("post flag(ex: $ msr mm id  -p text)")
                .alias("p"),
                )
            )
        .command(
            Command::new("delete")
            .usage("msr d")
            .description("delete latest post")
            .alias("d")
            .action(d),
            )
        .command(
            Command::new("icon")
            .usage("msr i")
            .description("timeline view avator")
            .alias("i")
            .action(icon_t),
            )
        .command(
            Command::new("accont")
            .usage("msr a {}")
            .description("account change, ex : $ msr a ~/test.toml, $ msr a -d(setting.toml)")
            .alias("a")
            .action(a),
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
    let post = mastodon.verify_credentials();
    println!("{:?}", post);
}

fn timeline() -> mammut::Result<()> {
    let mastodon = token();
    let length = &mastodon.get_home_timeline()?.initial_items.len();
    for n in 0..*length {
        let nn = &mastodon.get_home_timeline()?.initial_items[n];
        let user = &nn.account.username;
        let body = &nn.content;
        let reblog = &nn.reblog;
        if body.is_empty() == true {
            let ruser = &reblog.as_ref().unwrap().uri;
            let rbody = &reblog.as_ref().unwrap().content;
            println!("re:{} {:?} {:?}", user, ruser, rbody);
        } else {
            println!("{} {:?}", user, body);
        }
    }
    Ok(())
}

fn t(_c: &Context) {
    timeline().unwrap();
}

fn p(c: &Context) {
    let mastodon = token();
    let message = c.args[0].to_string();
    let status_b = StatusBuilder::new(format!("{}", message));
    let post = mastodon.new_status(status_b);
    println!("{:?}", post);
}

fn mention(c: &Context) {
    let mastodon = token();
    if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        let mid = Some(c.args[0].to_string());
        let status_b = StatusBuilder {
            status: status.to_string(),
            in_reply_to_id: mid,
            media_ids: None,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        let post = mastodon.new_status(status_b);
        println!("{:?}", post);
    }
}

#[allow(unused_must_use)]
fn delete() -> mammut::Result<()> {
    let mastodon = token();
    let n = &mastodon.get_home_timeline()?.initial_items[0];
    let user = &n.account.username;
    let body = &n.content;
    let id = &n.id;
    println!("delete -> {} {:?}", user, body);
    mastodon.delete_status(id);
    Ok(())
}

fn d(_c: &Context) {
    delete().unwrap();
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
}

fn media(c: &Context) {
    let mastodon = token();
    let file = c.args[0].to_string();
    if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        let s = Cow::Owned(String::from(text));
        let t = mastodon.media(
            MediaBuilder::new(file.into())
            .description(s)
            //.focus(200.0, 200.0)
            );
        let id = t.as_ref().unwrap();
        let mid = Some(vec![id.id.to_string()]);
        let status_b = StatusBuilder {
            status: status.to_string(),
            in_reply_to_id: None,
            media_ids: mid,
            sensitive: None,
            spoiler_text: None,
            visibility: None,
        };
        let post = mastodon.new_status(status_b);
        println!("{:?}", post);
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
    if let Ok(text) = c.string_flag("text") {
        let status = &*text.to_string();
        if b.is_none() {
            let opt: Option<i32> = None;
            println!("{:?}", opt);
        } else {
            match &*status {
                "id" => println!("{:#?}", id),
                "mid" => println!("{:#?}", mid),
                "user" => println!("{:#?}", user),
                "date" => println!("{:#?}", date),
                "body" => println!("{:#?}", body),
                "url" => println!("{:#?}", url),
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
            println!("{:#?} {:#?} {:#?} {:#?} {:?}", date, ntype, id, user, body);
            println!("{:#?}", mid);
            println!("{:#?}", url);
        }
    }
    Ok(())
}

fn nl(c: &Context) {
    notifylatest(c).unwrap();
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

fn icon(user: String) {
    use std::process::Command;
    let path = "/.config/msr/icon/";
    let file = path.to_string() + &user + &"-min.png";
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
            Command::new("img2sixel").arg(f).spawn().expect("sixel");
        }
        os_type::OSType::Ubuntu => {
            // apt-get install -y libsixel-bin
            Command::new("img2sixel").arg(f).spawn().expect("sixel");
        }
        _ => {
            if cfg!(target_os = "windows") {
                Command::new("img2sixel").arg(f).spawn().expect("sixel");
            };
        }
    }
}

fn icon_timeline() -> mammut::Result<()> {
    let mastodon = token();
    let length = &mastodon.get_home_timeline()?.initial_items.len();
    for n in 0..*length {
        let nn = &mastodon.get_home_timeline()?.initial_items[n];
        let avator = &nn.account.avatar_static;
        let user = &nn.account.username;
        let body = &nn.content;
        let reblog = &nn.reblog;
        let path = "/.config/msr/icon/";
        let fend = Path::new(&avator).extension().unwrap().to_str().unwrap();
        let file = path.to_string() + &user + &"." + &fend;
        let min = path.to_string() + &user + &"-min.png";
        let mut p = shellexpand::tilde("~").to_string();
        let mut f = shellexpand::tilde("~").to_string();
        let mut m = shellexpand::tilde("~").to_string();
        let mut i = shellexpand::tilde("~").to_string();
        p.push_str(&path);
        f.push_str(&file);
        m.push_str(&min);
        i.push_str(&file);
        match fs::create_dir_all(p) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
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
        let img = image::open(i).unwrap();
        let resized = image::imageops::resize(&img, 25, 25, image::imageops::Lanczos3);
        let check = Path::new(&m).exists();
        if check == false {
            resized.save(m).unwrap();
        }
        icon(user.to_string());
        if body.is_empty() == true {
            let ruser = &reblog.as_ref().unwrap().uri;
            let rbody = &reblog.as_ref().unwrap().content;
            println!("re:{} {:?} {:?}", user, ruser, rbody);
        } else {
            println!("{} {:?}", user, body);
        }
    }
    Ok(())
}

fn icon_t(_c: &Context) {
    icon_timeline().unwrap();
}

