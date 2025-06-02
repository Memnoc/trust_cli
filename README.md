# Trust - A dependency utility

Trust is a command that your run under `cargo` and outputs a `trust score` of the crates and their dependencies in your project.

## Why

As projects become larger and larger, it is almost inevitable a huge amount of dependencies will end up in your tree. This is not only true for highly vetted, large projects that are typically found in medium-large companies, but also for relatively small projects.

While using dependencies can greatly simplify your app by reducing the amount of code you need to write, while providing a level of abstraction developers can get on board with, the fact of the matter is: can you trust and can you keep track of the "health" of your dependencies?

## The solution

We would need to have pre-emptive checks for such crates, ideally before we install them in our projects, and even more so later on, while we rely on them for core functionality of our production-shipped apps.

## What

Enter `trust` - a CLI tool you can run at any moment against your main branch, that will analyze all of the dependencies in `Cargo.toml` and combine them in a single `trust score`.

## How

The data points taken into consideration are various, and sometimes not that black and white. Here is a temporary list:

- **Security concerns**
  - _How much unsafe can we find in the code?_
- **Test coverage**
  - _Is the code tested?_
- **Community support and size**
  - _Is there an active and significant community behind the crate?_
- **Big and/or famous crates association**
  - _Does any other big crate uses this code?_

## Tech Stack

### Crates.io API

The main API is at `https://crates.io/api/v1/`

- Crate info: `GET /crates/{crate_name}` - gives you basic metadata
- Crate versions: `GET /crates/{crate_name}/versions` - version history, release dates
- Download stats: `GET /crates/{crate_name}/downloads`- download statistics
- Dependencies: `GET /crates/{crate_name}/{version}/dependencies` - what it depends on
- Reverse dependencies: `GET /crates/{crate_name}/reverse_dependencies` - what depends on it
