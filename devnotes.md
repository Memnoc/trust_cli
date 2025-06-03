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
