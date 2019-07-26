# Arrr! mob

> Shiver me timbers! A command-line tool fer social codin', written in Rust

Includes `Co-authored by` lines in commits for ye automatically when ye collaborate on code. Use when pairin' wit' a matey or mobbin' wit' yer crew.

## Embark

1. `git clone` this ship
1. `cargo build` and `cargo install` for now
1. Add a `~/.git-copirates` file like this
    ```json
    {
      "copirates": {
        "cb": {
          "name": "Charlotte de Berry",
          "email": "berrydeath@england.pir"
        }
      }
    }

    ```
1. In your repo, `rmob embark`
1. `rmob sail` away!

---
```
Rmob 0.1.0
Arr! Git mobbing in Rust

USAGE:
    rmob <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    embark                Embark on rmob fer this git repo, call this once t' use rmob in yer git repo
    help                  Prints this message or the help of the given subcommand(s)
    prepare-commit-msg    Called from the git hook only
    sail                  Start pairin' or mobbin' by passin' a list of yer co-pirates te sail wit'
    solo                  Sail solo (short fer `rmob sail solo`)
```
---

Looted from the scallywags o'er at 'https://github.com/findmypast-oss/git-mob.

Mighty indebted!
