`msr` of mastodon cli client.

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

### example

```sh
$ msr p "test post"

$ msr m ./test.png

# mention
$ msr mm $id -p "$message"
$ id=`msr nl -o id|head -n 1|cut -d '"' -f 2`

# notify
$ msr n
$ msr nl -o id

# nofity-clear
$ msr n -c
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

### lib

mastodon api : https://github.com/XAMPPRocky/Mammut

rust cli : https://github.com/ksk001100/seahorse

