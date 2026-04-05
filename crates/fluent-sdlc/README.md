# fluent-sdlc

Umbrella crate that re-exports fluent CLI tool wrappers behind feature flags.

## Usage

```toml
[dependencies]
fluent-sdlc = { version = "0.1", features = ["git"] }
```

```rust
use fluent_sdlc::git::sync::git::init;
use std::path::Path;

let repo = init(Path::new("/tmp/repo")).run()?.into_repo();
```

## Features

- `git` (default) — re-exports `fluent-git`
