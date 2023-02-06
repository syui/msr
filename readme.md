`msr` of mastodon cli client.

and misskey client.

```sh
$ git clone https://github.com/syui/msr
$ cd msr
$ mkdir -p ~/.config/msr
$ cp config.toml.example ~/.config/msr/config.toml
$ vim ~/.config/msr/config.toml

$ cargo run
$ ./target/debug/msr s

# build
$ cargo build
$ ./target/debug/msr
```

err misskey env `~/.config/msr/config.toml`

```sh
misskey_token = "xxx"
misskey_base = "https://example.com"
misskey_api = "https://example.com/api"
misskey_stream = "wss://example.com/streaming"
```

### example

```sh
# status
$ msr s

# post
$ msr p "test post"

# translation(deepl api)
$ export api="xxx"
$ msr tt "test" -a $api

$ msr tt "test" -l ja
$ msr tt "テスト" -l en
$ msr tt "test"

# post translate
## en -> ja
$ msr p "test" -l en
## ja -> en
$ msr p "テスト" -l ja

# mention
$ msr mm $id -p "$message"
$ id=`msr nl -o id|head -n 1|cut -d '"' -f 2`

# media upload
$ msr m ./test.png
$ msr m ./test.png -p "text" -u

# media rep
$ msr m ./test.png -rid $id

# notify
$ msr n
$ msr nl -o id

# nofity-clear
$ msr n -c

# follow, unfollow
$ msr f @user@example.com
$ msr f @user@example.com -d

# search user
$ msr s -u @ai
$ msr s -i @ai

# search post
msr s -i "@syui@syui.cf" -t
msr s -i "@syui@syui.cf" -t -a

# fav and reblog
$ msr fa $id
$ msr fa $id -d
$ msr r $id
$ msr r $id -d
```

### icon

linux : img2sixel

mac : imgcat

not shown on tmux

```sh
# timeline
$ msr i
$ ls ~/.config/msr/icon/
```

### custom-prompt

> ~/.zshrc

```sh
my_mastodon() {
	source ~/.config/msr/msr.zsh
		export mastodon="%F{cyan}${icon_mastodon}%f : @${MASTODON_BASE##*/}"
#export mastodon="%F{cyan}${icon_mastodon}%f : @${MASTODON_USER}@${MASTODON_BASE##*/}"
}
autoload -Uz add-zsh-hook
add-zsh-hook precmd my_mastodon
```

### misskey

> ~/.config/msr/config.toml

```sh
misskey_token = "xxx"
misskey_base = "https://misky.syui.cf"
misskey_api = "https://misky.syui.cf/api"
misskey_stream = "wss://misky.syui.cf/streaming"
```

```sh
$ msr misky -p "hello world"
```

### lib

mastodon api : https://github.com/XAMPPRocky/Mammut

rust cli : https://github.com/ksk001100/seahorse

misskey api : https://github.com/coord-e/misskey-rs

