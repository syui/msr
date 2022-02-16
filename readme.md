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

### prompt

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
