# fluent-git

Fluent builder-pattern API for git operations.

## Examples

### Init, commit, branch

```rust
use fluent_git::sync::git::{init, open};
use std::path::Path;

// Create a repo
let result = init(Path::new("/tmp/repo")).branch("main").run()?;
let repo = result.into_repo();

// Stage and commit
repo.add().all().run()?;
repo.commit().message("first commit").run()?;

// Branch operations
repo.branch().create("feature").run()?;
repo.checkout().branch("feature").run()?;
```

### Clone with options

```rust
use fluent_git::sync::git::clone;
use std::path::Path;

let result = clone("https://github.com/user/repo.git")
    .depth(1)
    .branch("main")
    .into(Path::new("/tmp/repo"))
    .run()?;
```

### Async

```rust
use fluent_git::git::{init, open};

let result = init(Path::new("/tmp/repo")).run().await?;
let repo = result.into_repo();
repo.add().all().run_async().await?;
repo.commit().message("async commit").run_async().await?;
```

### Error handling

Each operation returns a domain-specific error:

```rust
use fluent_git::error::CommitError;

match repo.commit().message("x").run() {
    Ok(result) => println!("committed {}", result.short_sha),
    Err(CommitError::NothingToCommit) => println!("working tree clean"),
    Err(CommitError::IdentityNotConfigured) => println!("set user.name/email"),
    Err(e) => return Err(e.into()),
}
```

Combine operations with `GitError`:

```rust
use fluent_git::GitError;

fn workflow(repo: &fluent_git::types::Repo) -> Result<(), GitError> {
    repo.add().all().run()?;           // AddError -> GitError
    repo.commit().message("x").run()?; // CommitError -> GitError
    Ok(())
}
```

## Supported operations

add, branch, checkout, cherry-pick, clean, clone, commit, config, diff, init, log, merge, open, rebase, remote, reset, rev-parse, stash, status, tag, worktree

## Features

- `blocking` (default) — synchronous execution
- `tokio` — async execution
