mastodon cli client.

> ~/.config/msr/config.toml

```sh
$ mkdir -p ~/.config/msr
$ cp config.toml.example ~/.config/msr/config.toml
$ vim ~/.config/msr/config.toml

$ cargo run

$ ./target/debug/msr s
```

mastodon api : https://github.com/XAMPPRocky/Mammut

rust cli : https://github.com/ksk001100/seahorse

### example

```sh
$ msr p "test post"

$ msr m ./test.png
```
