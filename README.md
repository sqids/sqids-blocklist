# [Sqids](https://sqids.org/) Blocklist

[![Github Actions](https://img.shields.io/github/actions/workflow/status/sqids/sqids-blocklist/tests.yml?style=flat-square)](https://github.com/sqids/sqids-blocklist/actions)

This repository is used to manage the default Sqids blocklist.

It contains carefully chosen words that might not be appropriate to accidentally show up in auto-generated Sqids IDs.

## Get started

Running the program requires Rust v1.69+. The following will output a complete JSON blocklist. You can also copy it from the [output](output) folder.

```bash
cargo run
```

## Adding or Adjusting Data

**If you'd like to add a list or adjust a list, please submit PRs to the `next` branch.**

The `main` branch is frozen for this version of Sqids.

## Data

Lists of words are located in [data](data) folder.

Words are transformed to include these mutations:

- `i` → `1`
- `l` → `1`
- `o` → `0`

So, if the blocklist contains the word "low", it will also contain these variations: "l0w", "1ow" and "10w".

Other character replacements are not included because they're not that obvious when mixed in IDs.

Sqids ID matching that will cause a re-run (in order):

1. If the blocklisted word is <= 3 characters long, it has to match exactly
1. If the word has numbers, then match will happen only if ID starts or ends with that mutated word
1. Otherwise, only if the word is a substring of ID

Most common matches are from 4 letter words.
