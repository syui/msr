use std::env;

use mammut::{Data, Mastodon};

fn main() -> mammut::Result<()> {

    let data = Data {
        base: env::var("BASE").unwrap().into(),
        client_id: env::var("CLIENT_ID").unwrap().into(),
        client_secret: env::var("CLIENT_SECRET").unwrap().into(),
        redirect: env::var("REDIRECT").unwrap().into(),
        token: env::var("TOKEN").unwrap().into(),
    };

    let mastodon = Mastodon::from_data(data);
    println!("{:?}", mastodon.get_home_timeline()?.initial_items);
    Ok(())
}

