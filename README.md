# swapold

I'm talking about you, `rust-analyzer.json`. Can't even toggle feature flags lmao, how even.


# Installation

(assuming you use oh my zsh, I don't know otherwise, you can just ignore about the completion part actually cuz it's dead simple and shells will suggest files by default I guess)

```
cargo b --release
```

It will tell you where it puts the completion files to.

Make sure to create a place for it and tell zsh by doing this in your .zshrc:

```
fpath=(~/.local/share/swapold-completion $fpath)
```
Note, this has to be before you call `compinit`, if you use oh-my-zsh, just place it before the `source $ZSH/oh-my-zsh.sh` line.

Then,

```
cp target/release/build/swapold-b101b7697ea55c2c/out/_swapold ~/.local/share/swapol
d-completion
```

Then, 

```
cp target/release/swapold ~/.local/bin/
```

Then,

```
rm $ZSH_COMPDUMP
```

and 

```
omz reload
```



