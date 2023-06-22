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
