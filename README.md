# fluent-sdlc

Fluent builder-pattern wrappers for SDLC command-line tools in Rust.

## Crates

| Crate | Description |
|-------|-------------|
| [fluent-core](crates/fluent-core/) | Shared execution infrastructure |
| [fluent-git](crates/fluent-git/) | Fluent API for `git` |
| [fluent-sdlc](crates/fluent-sdlc/) | Umbrella re-export crate |

## Quick example

```rust
use fluent_git::sync::git::{init, open};

let result = init(Path::new("/tmp/my-repo")).branch("main").run()?;
let repo = result.into_repo();

repo.add().all().run()?;
repo.commit().message("initial").allow_empty().run()?;
repo.branch().create("feature").run()?;
repo.checkout().branch("feature").run()?;
```

## Design principles

- **Fluent builders, not flag soup.** Every git operation is a builder chain that reads like prose. No stringly-typed argument vectors.
- **Per-operation error types.** `commit().run()` returns `Result<CommitResult, CommitError>`, not a generic error. You pattern-match on domain-specific variants like `NothingToCommit` or `IdentityNotConfigured`.
- **Umbrella error for composition.** When combining operations, `GitError` collects all domain errors via `#[from]` so `?` just works.
- **Repo as proof of setup.** `init()` / `clone()` / `open()` return a `Repo` handle. All subsequent operations hang off it. You can't call `commit()` without first proving a repo exists.
- **Sync and async parity.** Feature-gated `blocking` (default) and `tokio` modes. Same builder API, same types, just `.await` on the async side.
- **Shared core.** `fluent-core` provides execution helpers and `CommandError` so new tool crates (gh, wrangler, etc.) reuse the same infrastructure.

## Usage

Depend on the tool crate you need:

```toml
[dependencies]
fluent-git = "0.1"
```

Or use the umbrella:

```toml
[dependencies]
fluent-sdlc = { version = "0.1", features = ["git"] }
```

For async:

```toml
[dependencies]
fluent-git = { version = "0.1", default-features = false, features = ["tokio"] }
```

## License

MIT
