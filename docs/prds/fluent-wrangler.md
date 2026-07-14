# PRD: fluent-wrangler

## Overview

A fluent builder-pattern Rust crate (`fluent-wrangler`) wrapping the Cloudflare Wrangler CLI (`wrangler`) for programmatic management of Cloudflare Workers, D1 databases, KV namespaces, R2 buckets, Pages projects, and related infrastructure. Designed as a composable building block for durable code workflows that automate Cloudflare deployments, infrastructure provisioning, and CI/CD pipelines.

## Motivation

Wrangler is the primary CLI for managing Cloudflare's developer platform. Programmatic access enables workflows that:

- Deploy Workers and Pages projects as part of automated CI/CD pipelines
- Provision and manage D1 databases, KV namespaces, and R2 buckets as infrastructure-as-code
- Rotate secrets across multiple Workers programmatically
- Execute D1 migrations and SQL as part of deployment pipelines
- Manage KV key-value data for cache warming, configuration distribution, and data seeding
- Upload and retrieve R2 objects for artifact storage and distribution
- Query deployment and version history for rollback decisions

## Scope

### In Scope (Priority 1 — Core Automation)

**Account:**
- `whoami` — retrieve authenticated user/account info (JSON output)

**Workers:**
- `deploy` — deploy a Worker (with dry-run support)
- `delete` — delete a Worker (with dry-run, force)
- `rollback` — rollback a deployment to a previous version

**Deployments:**
- `deployments list` — list recent deployments (JSON output)
- `deployments status` — view current production deployment state

**Versions:**
- `versions list` — list recent versions (JSON output)
- `versions view` — view version details
- `versions upload` — upload code as new version
- `versions deploy` — deploy versions with traffic split

**Secrets (Worker-scoped):**
- `secret list` — list secrets (JSON output, default format)
- `secret put` — create/update a secret (stdin-based)
- `secret delete` — delete a secret
- `secret bulk` — upload multiple secrets from JSON file

**D1 (SQL Database):**
- `d1 list` — list databases (JSON output)
- `d1 info` — get database info (JSON output)
- `d1 create` — create a database (location, jurisdiction options)
- `d1 delete` — delete a database
- `d1 execute` — execute SQL command or file (JSON output)

**KV Namespace:**
- `kv namespace create` — create a namespace
- `kv namespace list` — list namespaces (JSON output by default)
- `kv namespace delete` — delete a namespace
- `kv namespace rename` — rename a namespace

**KV Key:**
- `kv key put` — write a key-value pair
- `kv key get` — read a value
- `kv key list` — list keys (JSON output by default)
- `kv key delete` — delete a key

**KV Bulk:**
- `kv bulk get` — get multiple key-value pairs from file
- `kv bulk put` — upload multiple key-value pairs from file
- `kv bulk delete` — delete multiple keys from file

**R2 Bucket:**
- `r2 bucket create` — create a bucket (location, storage class, jurisdiction)
- `r2 bucket list` — list buckets
- `r2 bucket info` — get bucket info (JSON output)
- `r2 bucket delete` — delete a bucket

**R2 Object:**
- `r2 object get` — fetch an object (binary output to file)
- `r2 object put` — upload an object
- `r2 object delete` — delete an object

### In Scope (Priority 2 — Useful Automation)

**Pages:**
- `pages project list` — list projects (JSON output)
- `pages project create` — create a project
- `pages project delete` — delete a project
- `pages deploy` — deploy static assets
- `pages deployment list` — list deployments (JSON output)
- `pages deployment delete` — delete a deployment

**Queues:**
- `queues list` — list queues
- `queues create` — create a queue
- `queues update` — update a queue
- `queues delete` — delete a queue
- `queues info` — get queue info
- `queues consumer add` / `consumer remove` — manage Worker consumers
- `queues pause-delivery` / `resume-delivery` — control message delivery
- `queues purge` — purge all messages

**D1 Migrations:**
- `d1 migrations create` — create a new migration
- `d1 migrations list` — list unapplied migrations
- `d1 migrations apply` — apply unapplied migrations

**D1 Time Travel:**
- `d1 time-travel info` — get time travel info at point-in-time
- `d1 time-travel restore` — restore database to point-in-time

**Vectorize:**
- `vectorize list` — list indexes (JSON output)
- `vectorize create` / `delete` / `get` / `info`
- `vectorize insert` / `upsert` / `query` / `get-vectors` / `delete-vectors`

**Hyperdrive:**
- `hyperdrive list` / `create` / `delete` / `get` / `update`

**Workflows:**
- `workflows list` / `describe` / `delete` / `trigger`
- `workflows instances list` / `describe` / `terminate` / `pause` / `resume`

### Out of Scope

- Interactive/browser-based: `login`, `logout` (OAuth flows)
- Long-running processes: `dev`, `tail`, `pages dev`, `pages deployment tail`, `tunnel run`, `tunnel quick-start`
- Interactive scaffolding: `init`, `setup`, `pipelines setup`
- Browser openers: `docs`
- Shell completion: `complete`
- Code generation: `types`
- Experimental triggers: `triggers`
- Niche/beta services: tunnels, dispatch namespaces, containers, AI/AI Search, secrets store, pipelines, VPC, certificates/mTLS
- R2 bucket sub-management: sippy, catalog, notification, domain, dev-url, local-uploads, lifecycle, cors, lock

## Entry Point

```rust
use fluent_wrangler::Wrangler;

let w = Wrangler::new();

// Deploy a Worker
w.deploy()
    .script("src/index.ts")
    .name("my-worker")
    .dry_run()
    .run()?;

// Manage secrets
let secrets = w.secret().list()
    .name("my-worker")
    .run()?;

w.secret().put("API_KEY")
    .name("my-worker")
    .value("sk-secret-value")
    .run()?;

// D1 database operations
let databases = w.d1().list().run()?;

let result = w.d1().execute("my-db")
    .command("SELECT * FROM users WHERE active = 1")
    .remote()
    .run()?;

// KV operations
let namespaces = w.kv().namespace().list().run()?;

w.kv().key().put("cache-key")
    .namespace_id("abc123")
    .value("cached-value")
    .run()?;

// R2 operations
w.r2().bucket().create("my-artifacts")
    .location("wnam")
    .run()?;

w.r2().object().put("my-artifacts/release/v1.0.tar.gz")
    .file("./dist/release.tar.gz")
    .run()?;

// Pages deployment
w.pages().deploy()
    .directory("./dist")
    .project_name("my-site")
    .run()?;
```

`Wrangler` is stateless (like `ClaudeCode` and `Lima`). No lifetime parameters on builders. The nested entry points (`d1()`, `kv()`, `r2()`, `secret()`, `pages()`, etc.) return lightweight grouping structs that provide access to sub-operation builders.

## Spec Checklist

### Core Types

- [ ] `Wrangler` — entry point struct, implements `CliTool` for "wrangler"
- [ ] All builders own their data (no lifetime parameters)

### Global Flags

All builders should support (where applicable):
- [ ] `.config(path)` — `--config` path to wrangler config file
- [ ] `.cwd(path)` — `--cwd` run as if in specified directory
- [ ] `.env_name(name)` — `-e/--env` environment to use
- [ ] `.env_file(path)` — `--env-file` path to .env file(s) (multi-call)
- [ ] Pages builders omit `config` and `env_name` (not supported by `wrangler pages`)

### Account

- [ ] `WhoAmIBuilder` — no required args, returns `WhoAmIInfo`

### Workers

- [ ] `DeployBuilder` — script (positional), name, dry_run, triggers, tag, message, minify, keep_vars, assets, no_bundle, outdir, upload_source_maps
- [ ] `DeleteBuilder` — name (positional), dry_run, force
- [ ] `RollbackBuilder` — version_id (positional), message

### Deployments

- [ ] `DeploymentsGroup` — entry with `list()`, `status()`
- [ ] `DeploymentsListBuilder` — name, returns `Vec<DeploymentInfo>`
- [ ] `DeploymentsStatusBuilder` — name

### Versions

- [ ] `VersionsGroup` — entry with `list()`, `view(id)`, `upload()`, `deploy()`
- [ ] `VersionsListBuilder` — name, returns `Vec<VersionInfo>`
- [ ] `VersionsViewBuilder` — version_id, name
- [ ] `VersionsUploadBuilder` — script (positional), name, tag, message
- [ ] `VersionsDeployBuilder` — version_specs (multi-call for traffic split), name, message

### Secrets

- [ ] `SecretGroup` — entry with `list()`, `put(key)`, `delete(key)`, `bulk()`
- [ ] `SecretListBuilder` — name (worker name), returns `Vec<SecretInfo>`
- [ ] `SecretPutBuilder` — key, name (worker name), value (passed via stdin)
- [ ] `SecretDeleteBuilder` — key, name (worker name)
- [ ] `SecretBulkBuilder` — file path, name (worker name)

### D1

- [ ] `D1Group` — entry with `list()`, `info(name)`, `create(name)`, `delete(name)`, `execute(db)`, `migrations()`, `time_travel()`
- [ ] `D1ListBuilder` — returns `Vec<D1DatabaseInfo>`
- [ ] `D1InfoBuilder` — database name, returns `D1DatabaseInfo`
- [ ] `D1CreateBuilder` — database name, location, jurisdiction, binding, update_config, use_remote
- [ ] `D1DeleteBuilder` — database name
- [ ] `D1ExecuteBuilder` — database name, command/file, local/remote/preview, yes, persist_to, returns `D1ExecuteResult`
- [ ] `D1MigrationsGroup` — entry with `create(db, message)`, `list(db)`, `apply(db)`
- [ ] `D1MigrationsCreateBuilder` — database, message
- [ ] `D1MigrationsListBuilder` — database
- [ ] `D1MigrationsApplyBuilder` — database, local/remote/preview
- [ ] `D1TimeTravelGroup` — entry with `info(db)`, `restore(db)`
- [ ] `D1TimeTravelInfoBuilder` — database, timestamp/bookmark
- [ ] `D1TimeTravelRestoreBuilder` — database, timestamp/bookmark

### KV

- [ ] `KvGroup` — entry with `namespace()`, `key()`, `bulk()`
- [ ] `KvNamespaceGroup` — entry with `create(ns)`, `list()`, `delete()`, `rename()`
- [ ] `KvNamespaceCreateBuilder` — namespace title
- [ ] `KvNamespaceListBuilder` — returns `Vec<KvNamespaceInfo>`
- [ ] `KvNamespaceDeleteBuilder` — binding/namespace_id
- [ ] `KvNamespaceRenameBuilder` — old namespace, new title
- [ ] `KvKeyGroup` — entry with `put(key)`, `get(key)`, `list()`, `delete(key)`
- [ ] `KvKeyPutBuilder` — key, value/path, binding/namespace_id, preview, local/remote, persist_to, metadata, expiration, expiration_ttl
- [ ] `KvKeyGetBuilder` — key, binding/namespace_id, preview, local/remote, persist_to
- [ ] `KvKeyListBuilder` — binding/namespace_id, prefix, preview, local/remote, persist_to, returns `Vec<KvKeyInfo>`
- [ ] `KvKeyDeleteBuilder` — key, binding/namespace_id, preview, local/remote, persist_to
- [ ] `KvBulkGroup` — entry with `get(file)`, `put(file)`, `delete(file)`
- [ ] `KvBulkGetBuilder` — file, binding/namespace_id, preview, local/remote, persist_to
- [ ] `KvBulkPutBuilder` — file, binding/namespace_id, preview, local/remote, persist_to
- [ ] `KvBulkDeleteBuilder` — file, binding/namespace_id, preview, local/remote, persist_to

### R2

- [ ] `R2Group` — entry with `bucket()`, `object()`
- [ ] `R2BucketGroup` — entry with `create(name)`, `list()`, `info(name)`, `delete(name)`
- [ ] `R2BucketCreateBuilder` — name, location, storage_class, jurisdiction, binding, update_config, use_remote
- [ ] `R2BucketListBuilder` — returns `Vec<R2BucketInfo>` (text-parsed or JSON)
- [ ] `R2BucketInfoBuilder` — name, returns `R2BucketInfo`
- [ ] `R2BucketDeleteBuilder` — name
- [ ] `R2ObjectGroup` — entry with `get(path)`, `put(path)`, `delete(path)`
- [ ] `R2ObjectGetBuilder` — object key, file (output path), pipe
- [ ] `R2ObjectPutBuilder` — object key, file (input path), content_type, content_disposition, content_encoding, content_language, cache_control, expires
- [ ] `R2ObjectDeleteBuilder` — object key(s)

### Pages

- [ ] `PagesGroup` — entry with `project()`, `deploy()`, `deployment()`
- [ ] `PagesProjectGroup` — entry with `list()`, `create(name)`, `delete(name)`
- [ ] `PagesProjectListBuilder` — returns `Vec<PagesProjectInfo>`
- [ ] `PagesProjectCreateBuilder` — name, production_branch
- [ ] `PagesProjectDeleteBuilder` — name
- [ ] `PagesDeployBuilder` — directory, project_name, branch, commit_hash, commit_message, commit_dirty
- [ ] `PagesDeploymentGroup` — entry with `list()`, `delete(id)`
- [ ] `PagesDeploymentListBuilder` — project_name, returns `Vec<PagesDeploymentInfo>`
- [ ] `PagesDeploymentDeleteBuilder` — deployment_id, project_name

### Queues

- [ ] `QueuesGroup` — entry with `list()`, `create(name)`, `update(name)`, `delete(name)`, `info(name)`, `consumer()`, `pause_delivery(name)`, `resume_delivery(name)`, `purge(name)`
- [ ] `QueuesListBuilder` — returns queue listing
- [ ] `QueuesCreateBuilder` — name, delivery_delay_secs, message_retention_period_secs
- [ ] `QueuesUpdateBuilder` — name, delivery_delay_secs, message_retention_period_secs
- [ ] `QueuesDeleteBuilder` — name
- [ ] `QueuesInfoBuilder` — name
- [ ] `QueuesConsumerGroup` — entry with `add(queue, script)`, `remove(queue, script)`
- [ ] `QueuesConsumerAddBuilder` — queue name, script name, batch_size, batch_timeout, message_retries, dead_letter_queue, max_concurrency
- [ ] `QueuesConsumerRemoveBuilder` — queue name, script name
- [ ] `QueuesPauseDeliveryBuilder` — name
- [ ] `QueuesResumeDeliveryBuilder` — name
- [ ] `QueuesPurgeBuilder` — name

### Vectorize (Priority 2)

- [ ] `VectorizeGroup` — entry with `list()`, `create(name)`, `delete(name)`, `get(name)`, `info(name)`, `insert(name)`, `upsert(name)`, `query(name)`, `get_vectors(name)`, `delete_vectors(name)`
- [ ] `VectorizeListBuilder` — returns `Vec<VectorizeIndexInfo>`
- [ ] `VectorizeCreateBuilder` — name, dimensions, metric, preset
- [ ] `VectorizeDeleteBuilder` — name
- [ ] `VectorizeGetBuilder` — name
- [ ] `VectorizeInfoBuilder` — name
- [ ] `VectorizeInsertBuilder` — name, file (ndjson)
- [ ] `VectorizeUpsertBuilder` — name, file (ndjson)
- [ ] `VectorizeQueryBuilder` — name, vector, top_k, return_values, return_metadata, namespace, filter
- [ ] `VectorizeGetVectorsBuilder` — name, ids
- [ ] `VectorizeDeleteVectorsBuilder` — name, ids

### Hyperdrive (Priority 2)

- [ ] `HyperdriveGroup` — entry with `list()`, `create(name)`, `delete(id)`, `get(id)`, `update(id)`
- [ ] `HyperdriveListBuilder`
- [ ] `HyperdriveCreateBuilder` — name, connection_string, host, port, database, user, password, scheme, caching_disabled, max_age, swr
- [ ] `HyperdriveDeleteBuilder` — id
- [ ] `HyperdriveGetBuilder` — id
- [ ] `HyperdriveUpdateBuilder` — id (same options as create)

### Workflows (Priority 2)

- [ ] `WorkflowsGroup` — entry with `list()`, `describe(name)`, `delete(name)`, `trigger(name)`, `instances()`
- [ ] `WorkflowsListBuilder`
- [ ] `WorkflowsDescribeBuilder` — name
- [ ] `WorkflowsDeleteBuilder` — name
- [ ] `WorkflowsTriggerBuilder` — name, params (JSON string)
- [ ] `WorkflowsInstancesGroup` — entry with `list(name)`, `describe(name)`, `terminate(name, id)`, `pause(name, id)`, `resume(name, id)`
- [ ] `WorkflowsInstancesListBuilder` — workflow name
- [ ] `WorkflowsInstancesDescribeBuilder` — workflow name, instance_id
- [ ] `WorkflowsInstancesTerminateBuilder` — workflow name, instance_id
- [ ] `WorkflowsInstancesPauseBuilder` — workflow name, instance_id
- [ ] `WorkflowsInstancesResumeBuilder` — workflow name, instance_id

### Output Types (all Serialize + Deserialize)

- [ ] `WhoAmIInfo` — account_id, account_name, token_permissions, etc.
- [ ] `DeploymentInfo` — id, created_on, author, source (version details)
- [ ] `VersionInfo` — id, metadata, number, created_on
- [ ] `SecretInfo` — name, type
- [ ] `D1DatabaseInfo` — uuid, name, created_at, version, num_tables, file_size, running_in_region
- [ ] `D1ExecuteResult` — results (array of row objects), success, meta (changes, duration, rows_read, rows_written)
- [ ] `KvNamespaceInfo` — id, title, supports_url_encoding
- [ ] `KvKeyInfo` — name, expiration, metadata
- [ ] `R2BucketInfo` — name, creation_date, location, storage_class
- [ ] `PagesProjectInfo` — name, subdomain, domains, production_branch, created_on, latest_deployment
- [ ] `PagesDeploymentInfo` — id, environment, project_name, url, created_on, latest_stage
- [ ] `VectorizeIndexInfo` — name, dimensions, metric, description, created_on, modified_on
- [ ] `QueueInfo` — name, created_on, modified_on, producers, consumers

### Error Types

- [ ] `WranglerError` — umbrella enum
- [ ] `AccountError` — NotAuthenticated, Command, Io
- [ ] `WorkerError` — NotFound, AlreadyExists, DeployFailed, Command, Io
- [ ] `D1Error` — NotFound, AlreadyExists, SqlError, MigrationFailed, Command, Io
- [ ] `KvError` — NamespaceNotFound, KeyNotFound, Command, Io
- [ ] `R2Error` — BucketNotFound, BucketAlreadyExists, ObjectNotFound, Command, Io
- [ ] `SecretError` — NotFound, Command, Io
- [ ] `PagesError` — ProjectNotFound, DeployFailed, Command, Io
- [ ] `QueuesError` — NotFound, AlreadyExists, Command, Io
- [ ] `VectorizeError` — IndexNotFound, AlreadyExists, Command, Io
- [ ] `HyperdriveError` — NotFound, Command, Io
- [ ] `WorkflowsError` — NotFound, InstanceNotFound, Command, Io
- [ ] `DeploymentError` — NotFound, Command, Io
- [ ] `VersionError` — NotFound, Command, Io

### Integration

- [ ] `CliTool` impl for `Wrangler` (program = "wrangler")
- [ ] Workspace dependency in root `Cargo.toml`
- [ ] Feature flag in `fluent-sdlc` (`wrangler = ["dep:fluent-wrangler"]`)
- [ ] Sync execution (`blocking` feature)
- [ ] Async execution (`tokio` feature)

### File Structure

```
crates/fluent-wrangler/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── ops/
    │   ├── mod.rs
    │   ├── account.rs          # WhoAmIBuilder
    │   ├── worker.rs           # DeployBuilder, DeleteBuilder, RollbackBuilder
    │   ├── deployment.rs       # DeploymentsListBuilder, DeploymentsStatusBuilder
    │   ├── version.rs          # VersionsListBuilder, VersionsViewBuilder, etc.
    │   ├── secret.rs           # SecretListBuilder, SecretPutBuilder, etc.
    │   ├── d1.rs               # D1 builders (list, info, create, delete, execute)
    │   ├── d1_migration.rs     # D1MigrationsCreateBuilder, ListBuilder, ApplyBuilder
    │   ├── d1_time_travel.rs   # D1TimeTravelInfoBuilder, RestoreBuilder
    │   ├── kv_namespace.rs     # KvNamespace builders
    │   ├── kv_key.rs           # KvKey builders
    │   ├── kv_bulk.rs          # KvBulk builders
    │   ├── r2_bucket.rs        # R2Bucket builders
    │   ├── r2_object.rs        # R2Object builders
    │   ├── pages.rs            # Pages project, deploy, deployment builders
    │   ├── queues.rs           # Queues builders
    │   ├── vectorize.rs        # Vectorize builders
    │   ├── hyperdrive.rs       # Hyperdrive builders
    │   └── workflows.rs        # Workflows builders
    ├── error/
    │   ├── mod.rs              # WranglerError umbrella
    │   ├── account.rs
    │   ├── worker.rs
    │   ├── d1.rs
    │   ├── kv.rs
    │   ├── r2.rs
    │   ├── secret.rs
    │   ├── pages.rs
    │   ├── queues.rs
    │   ├── vectorize.rs
    │   ├── hyperdrive.rs
    │   ├── workflows.rs
    │   ├── deployment.rs
    │   └── version.rs
    ├── types/
    │   ├── mod.rs
    │   ├── wrangler.rs         # Wrangler entry point + group structs
    │   ├── account.rs          # WhoAmIInfo
    │   ├── deployment.rs       # DeploymentInfo
    │   ├── version.rs          # VersionInfo
    │   ├── secret.rs           # SecretInfo
    │   ├── d1.rs               # D1DatabaseInfo, D1ExecuteResult
    │   ├── kv.rs               # KvNamespaceInfo, KvKeyInfo
    │   ├── r2.rs               # R2BucketInfo
    │   ├── pages.rs            # PagesProjectInfo, PagesDeploymentInfo
    │   ├── queues.rs           # QueueInfo
    │   ├── vectorize.rs        # VectorizeIndexInfo
    │   └── workflows.rs        # WorkflowInfo
    ├── sync/wrangler/ops/
    │   └── *.rs                # Sync implementations
    └── wrangler/ops/
        └── *.rs                # Async implementations
```

## Design Notes

### Output Parsing Strategy

Wrangler has inconsistent JSON support across commands:

1. **Explicit `--json` flag**: `whoami`, `d1 list`, `d1 info`, `d1 execute`, `deployments list`, `versions list`, `pages project list`, `pages deployment list`, `r2 bucket info`, `vectorize list` — parse via serde deserialization.

2. **`--format json`**: `secret list` (default format is already JSON), `tail` (out of scope) — use `--format json` flag, parse via serde.

3. **JSON by default**: `kv namespace list`, `kv key list` — output JSON arrays without any flag, parse via serde.

4. **No JSON support**: Most create/delete/update operations output human-readable text. For these, capture stderr/stdout for error detection but don't parse structured output. Success is determined by exit code.

5. **Binary output**: `r2 object get` outputs raw binary — pipe to file, don't parse.

For commands without JSON output, builders return `()` on success and the error type on failure, using exit code to determine success.

### Global Flags via Trait or Base Struct

Global flags (`--config`, `--cwd`, `--env`, `--env-file`) are shared across nearly all builders. Implement via a `GlobalFlags` struct embedded in each builder, with chainable methods. Pages builders exclude `--config` and `--env` since those flags are not supported by `wrangler pages`.

### Secret Input via Stdin

`wrangler secret put` reads the secret value from stdin, not from a CLI argument. The builder must pipe the value through stdin of the child process rather than passing it as a flag. Provide a `.value(str)` method that handles this internally.

### KV Namespace Targeting

KV operations target a namespace via either `--binding` (config-based) or `--namespace-id` (explicit). Builders should support both. The `--preview` flag targets the preview namespace when using bindings.

### R2 Object Paths

R2 object commands use a combined `bucket/key` path format: `wrangler r2 object get my-bucket/path/to/file.txt`. The builder accepts the full path as the positional argument.

### D1 Execute Modes

`d1 execute` supports `--local` (local SQLite), `--remote` (production), and `--preview` (preview database). Default is interactive prompt — for programmatic use, always specify one explicitly. The `--yes` flag skips confirmation prompts.

### Non-Interactive Defaults

For commands that prompt for confirmation (delete operations, `d1 execute`), builders should include `--force` or `-y`/`--yes` flags where available. Document which operations are destructive and require explicit confirmation bypass.

### Authentication

The crate does not handle authentication. Users must authenticate via `wrangler login` (interactive, out of scope), `CLOUDFLARE_API_TOKEN` environment variable, or `CLOUDFLARE_API_KEY` + `CLOUDFLARE_EMAIL` environment variables before using the crate. Document this in the crate-level documentation.
