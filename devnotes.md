# Dependencies

## Serde/serde.json

De-serializing `JSON` files which essential for API responses
URL: [serde](https://docs.rs/serde/latest/serde/)
URL: [serde_json](https://docs.rs/serde_json/latest/serde_json/)

## toml

To parse the [TOML](https://github.com/toml-lang/toml) files, essential core functionality of `trust-cli`
URL: [toml](https://docs.rs/toml/latest/toml/)

## ureq

Minimal and safe HTTP client library - normally I would use `reqwest` but for now we can keep it simple
URL: [ureq](https://github.com/algesten/ureq)

## clap

Highly customizable CLI library
A simpler version could use `std::env::args()`

URL: [clap](https://docs.rs/clap/latest/clap/)

## directories

Tiny library for finding and storing data - useful to store the examples and the tests
We may need cross-platform cache directories

URL: [directories](https://docs.rs/directories/latest/directories/)

## chrono

Handles timestamps in cache - a deps free solution here would be `std::time`

URL: [chrono](https://docs.rs/chrono/latest/chrono/)

# Data Points

## Crates.io

So far, it seems there is indeed a restful API that exposed all crates as I would expect.
Various discussion on reddit led me here: https://github.com/hcpl/crates.io-http-api-reference?tab=readme-ov-file

If I ping, for example, `https:://crates.io/api/v1/crates` I get in fact JSON back - good news!

From this main endpoint, we want:

- info (description)
- version (crate version)
- download stats
- dependencies

## GitHub

Here we have full REST API so it's easy. We want mostly popularity scores, such as:

- last commit
- issues/PR activity
- stars/watchers

`https://api.github.com/repos/{owner}/{repo}`

```rust
let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
let response: GitHubRepo = ureq::get(&url)
    .set("User-Agent", "cargo-trust")  // Required!
    .call()?
    .into_json()?;
```

## RustSec Advisory Database

This seems to be a git repository with structured TOML files - it's a first for me
