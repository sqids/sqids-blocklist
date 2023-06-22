# [Sqids](https://sqids.org/) Blacklist

[![Github Actions](https://img.shields.io/github/actions/workflow/status/sqids/sqids-blacklist/tests.yml?style=flat-square)](https://github.com/sqids/sqids-blacklist/actions)

> 🏗️ This is still a work in progress. Do not use in production.

This repository is used to manage the default Sqids blacklist.

It contains carefully chosen words that might not be appropriate to accidentally show up in auto-generated Sqids IDs.

## Get started

Running the program requires Rust v1.69+. This will output a JSON list of the blacklist:

```bash
cargo run
```

## Data

Lists of words are located in [data](data) folder.

Words are transformed to include these mutations:

- `i` → `1`
- `l` → `1`
- `o` → `0`

So, if the blacklist contains the word "low", it will also contain these variations: "l0w", "1ow" and "10w".

Other character replacements are not included because they're not that obvious when mixed in IDs.

Sqids ID matching that will cause a re-run (in order):

1. If the blacklisted word is <= 3 characters long, it has to match exactly
1. If the word has numbers, then match will happen only if ID starts or ends with that mutated word
1. Otherwise, only if the word is a substring of ID

Most common matches are from 4 letter words.