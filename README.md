# Arrr! mob

> Shiver me timbers! A command-line tool fer social codin', written in Rust

[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg?style=flat-square)](https://www.gnu.org/licenses/gpl-3.0)
[![Arrr: Arrr](https://img.shields.io/badge/pirate-arr-yellow)]()

Includes `Co-authored by` lines in commits for ye automatically when ye collaborate on code. Use when pairin' wit' a matey or mobbin' wit' yer crew.

## Embark

1. `git clone` this ship
1. `cargo build` and `cargo install --path .` for now
1. Add co-pirates by running:
    ```bash
    rmob copirate enlist cb "Charlotte de Berry" "berrydeath@england.pir"
    ```
1. In your repo, `rmob embark`
1. `rmob sail` away!

---
```
Rmob 0.1.0
Arr! Git mobbing in Rust

USAGE:
    rmob [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --git-copirates-file <git_copirates_file>     [env: GIT_COPIRATES_FILE=]  [default: .git-copirates]

SUBCOMMANDS:
    copirate              Edit copirates list
    embark                Embark on rmob fer this git repo, call this once t' use rmob in yer git repo
    help                  Prints this message or the help of the given subcommand(s)
    prepare-commit-msg    Called from the git hook only
    sail                  Start pairin' or mobbin' by passin' a list of yer co-pirates te sail wit'
    solo                  Sail solo (short fer `rmob sail solo`)
```
---

Looted from the scallywags o'er at 'https://github.com/findmypast-oss/git-mob.

Mighty indebted!
