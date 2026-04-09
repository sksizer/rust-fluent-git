# fluent-wrangler CLI Research

**CLI**: `wrangler` (Cloudflare Workers CLI)
**Version**: 4.80.0
**Install**: `npm install -g wrangler` / `pnpm add -g wrangler`

## Global Flags

All subcommands accept these flags:

| Flag | Type | Description |
|------|------|-------------|
| `-c, --config` | string | Path to Wrangler configuration file |
| `--cwd` | string | Run as if started in specified directory |
| `-e, --env` | string | Environment to use for operations |
| `--env-file` | array | Path to .env file(s) to load |
| `-h, --help` | boolean | Show help |
| `-v, --version` | boolean | Show version number |

Note: `pages` subcommands do NOT support `-c, --config` or `-e, --env`.

## Operation Groups

### 1. Account & Auth

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler whoami` | Retrieve user information | `--json` |
| `wrangler login` | Login to Cloudflare (interactive/browser) | No |
| `wrangler logout` | Logout from Cloudflare | No |
| `wrangler auth token` | Retrieve current authentication token | No |

### 2. Workers (Deploy/Manage)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler deploy [script]` | Deploy a Worker | No (has `--dry-run`) |
| `wrangler delete [name]` | Delete a Worker | No (has `--dry-run`, `--force`) |
| `wrangler rollback [version-id]` | Rollback a deployment | No |
| `wrangler tail [worker]` | Start log tailing session (streaming) | `--format json` |
| `wrangler init [name]` | Initialize a basic Worker (interactive) | No |
| `wrangler setup` | Setup project (interactive) | No |
| `wrangler types [path]` | Generate types from config | No |
| `wrangler triggers` | Update triggers (experimental) | No |

### 3. Deployments & Versions

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler deployments list` | List 10 most recent deployments | `--json` |
| `wrangler deployments status` | View current production state | No |
| `wrangler versions list` | List 10 most recent versions | `--json` |
| `wrangler versions view <id>` | View details of a specific version | No |
| `wrangler versions upload [script]` | Upload code as new version | No |
| `wrangler versions deploy [specs..]` | Deploy versions with traffic split | No |
| `wrangler versions secret` | Manage version secrets | No |

### 4. Secrets (Worker-scoped)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler secret list` | List all secrets for a Worker | `--format json` (default) |
| `wrangler secret put <key>` | Create/update a secret (reads from stdin) | No |
| `wrangler secret delete <key>` | Delete a secret | No |
| `wrangler secret bulk [file]` | Upload multiple secrets from JSON file | No |

### 5. D1 (SQL Database)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler d1 list` | List all D1 databases | `--json` |
| `wrangler d1 info <name>` | Get database info (size, state) | `--json` |
| `wrangler d1 create <name>` | Create a new D1 database | No |
| `wrangler d1 delete <name>` | Delete a D1 database | No |
| `wrangler d1 execute <db>` | Execute SQL command/file | `--json` |
| `wrangler d1 export <name>` | Export database as .sql | No |
| `wrangler d1 time-travel info <db>` | Time Travel info at point-in-time | No |
| `wrangler d1 time-travel restore <db>` | Restore database to point-in-time | No |
| `wrangler d1 migrations create <db> <msg>` | Create a new migration | No |
| `wrangler d1 migrations list <db>` | List unapplied migrations | No |
| `wrangler d1 migrations apply <db>` | Apply unapplied migrations | No |
| `wrangler d1 insights <name>` | Query analytics (experimental) | No |

`d1 execute` options: `--command`, `--file`, `--local`, `--remote`, `--preview`, `--yes`, `--persist-to`

`d1 create` options: `--location` (weur/eeur/apac/oc/wnam/enam), `--jurisdiction` (eu/fedramp), `--binding`, `--update-config`, `--use-remote`

### 6. KV (Key-Value Store)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler kv namespace create <ns>` | Create a new namespace | No |
| `wrangler kv namespace list` | List all KV namespaces | Outputs JSON by default |
| `wrangler kv namespace delete [ns]` | Delete a namespace | No |
| `wrangler kv namespace rename [old]` | Rename a namespace | No |
| `wrangler kv key put <key> [value]` | Write a key/value pair | No |
| `wrangler kv key list` | List all keys in namespace | Outputs JSON by default |
| `wrangler kv key get <key>` | Read a single value | No |
| `wrangler kv key delete <key>` | Delete a key | No |
| `wrangler kv bulk get <file>` | Get multiple key-value pairs | No |
| `wrangler kv bulk put <file>` | Upload multiple key-value pairs | No |
| `wrangler kv bulk delete <file>` | Delete multiple key-value pairs | No |

KV key/namespace operations use `--binding`, `--namespace-id`, `--preview`, `--local`, `--remote`, `--persist-to` to target namespaces.

### 7. R2 (Object Storage)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler r2 bucket create <name>` | Create a new bucket | No |
| `wrangler r2 bucket list` | List R2 buckets | No explicit flag; outputs table |
| `wrangler r2 bucket info <bucket>` | Get bucket information | `--json` |
| `wrangler r2 bucket delete <bucket>` | Delete a bucket | No |
| `wrangler r2 bucket update` | Update bucket state | No |
| `wrangler r2 bucket sippy` | Manage Sippy incremental migration | No |
| `wrangler r2 bucket catalog` | Manage data catalog (Iceberg REST) | No |
| `wrangler r2 bucket notification` | Manage event notifications | No |
| `wrangler r2 bucket domain` | Manage custom domains | No |
| `wrangler r2 bucket dev-url` | Manage r2.dev public URL | No |
| `wrangler r2 bucket local-uploads` | Manage local uploads config | No |
| `wrangler r2 bucket lifecycle` | Manage lifecycle rules | No |
| `wrangler r2 bucket cors` | Manage CORS configuration | No |
| `wrangler r2 bucket lock` | Manage lock rules | No |
| `wrangler r2 object get <path>` | Fetch an object | Binary output |
| `wrangler r2 object put <path>` | Create an object | No |
| `wrangler r2 object delete <path>` | Delete an object | No |
| `wrangler r2 sql query <warehouse> <query>` | Execute SQL query (open beta) | No |

`r2 bucket create` options: `--location`, `--storage-class`, `--jurisdiction`, `--binding`, `--update-config`, `--use-remote`

### 8. Queues

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler queues list` | List queues | No explicit flag |
| `wrangler queues create <name>` | Create a queue | No |
| `wrangler queues update <name>` | Update a queue | No |
| `wrangler queues delete <name>` | Delete a queue | No |
| `wrangler queues info <name>` | Get queue information | No explicit flag |
| `wrangler queues consumer add <q> <script>` | Add Worker consumer | No |
| `wrangler queues consumer remove <q> <script>` | Remove Worker consumer | No |
| `wrangler queues consumer http` | Configure HTTP pull consumers | No |
| `wrangler queues consumer worker` | Configure Worker consumers | No |
| `wrangler queues pause-delivery <name>` | Pause message delivery | No |
| `wrangler queues resume-delivery <name>` | Resume message delivery | No |
| `wrangler queues purge <name>` | Purge messages | No |
| `wrangler queues subscription` | Manage event subscriptions | No |

`queues create` options: `--delivery-delay-secs`, `--message-retention-period-secs`

### 9. Pages

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler pages project list` | List Pages projects | `--json` |
| `wrangler pages project create <name>` | Create a Pages project | No |
| `wrangler pages project delete <name>` | Delete a Pages project | No |
| `wrangler pages deploy [directory]` | Deploy static assets | No |
| `wrangler pages deployment list` | List deployments | `--json` |
| `wrangler pages deployment delete <id>` | Delete a deployment | No |
| `wrangler pages deployment tail [deploy]` | Tail deployment logs (streaming) | No |
| `wrangler pages dev [dir] [cmd]` | Local dev server (interactive) | No |
| `wrangler pages secret` | Manage Pages secrets | No |
| `wrangler pages download` | Download project settings | No |
| `wrangler pages functions` | Pages Functions helpers | No |

Note: Pages commands do NOT support `-c/--config` or `-e/--env` global flags.

### 10. Vectorize

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler vectorize list` | List indexes | `--json` |
| `wrangler vectorize create <name>` | Create an index | No |
| `wrangler vectorize delete <name>` | Delete an index | No |
| `wrangler vectorize get <name>` | Get index by name | No |
| `wrangler vectorize info <name>` | Get additional details | No |
| `wrangler vectorize list-vectors <name>` | List vector IDs | No |
| `wrangler vectorize query <name>` | Query an index | No |
| `wrangler vectorize insert <name>` | Insert vectors | No |
| `wrangler vectorize upsert <name>` | Upsert vectors | No |
| `wrangler vectorize get-vectors <name>` | Get vectors | No |
| `wrangler vectorize delete-vectors <name>` | Delete vectors | No |
| `wrangler vectorize create-metadata-index <name>` | Enable metadata filtering | No |
| `wrangler vectorize list-metadata-index <name>` | List metadata indexes | No |
| `wrangler vectorize delete-metadata-index <name>` | Delete metadata indexes | No |

### 11. Hyperdrive

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler hyperdrive list` | List configs | No explicit flag |
| `wrangler hyperdrive create <name>` | Create a config | No |
| `wrangler hyperdrive delete <id>` | Delete a config | No |
| `wrangler hyperdrive get <id>` | Get a config | No |
| `wrangler hyperdrive update <id>` | Update a config | No |

### 12. Workflows

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler workflows list` | List workflows | No explicit flag |
| `wrangler workflows describe <name>` | Describe workflow | No |
| `wrangler workflows delete <name>` | Delete workflow | No |
| `wrangler workflows trigger <name> [params]` | Trigger a workflow instance | No |
| `wrangler workflows instances list <name>` | List instances | No |
| `wrangler workflows instances describe <name> [id]` | Describe instance | No |
| `wrangler workflows instances send-event <name> <id>` | Send event to instance | No |
| `wrangler workflows instances terminate <name> <id>` | Terminate instance | No |
| `wrangler workflows instances restart <name> <id>` | Restart instance | No |
| `wrangler workflows instances pause <name> <id>` | Pause instance | No |
| `wrangler workflows instances resume <name> <id>` | Resume instance | No |

### 13. Dispatch Namespaces (Workers for Platforms)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler dispatch-namespace list` | List all dispatch namespaces | No |
| `wrangler dispatch-namespace get <name>` | Get namespace info | No |
| `wrangler dispatch-namespace create <name>` | Create namespace | No |
| `wrangler dispatch-namespace delete <name>` | Delete namespace | No |
| `wrangler dispatch-namespace rename <old> <new>` | Rename namespace | No |

### 14. Tunnels (Experimental)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler tunnel list` | List all tunnels | No |
| `wrangler tunnel create <name>` | Create a tunnel | No |
| `wrangler tunnel delete <tunnel>` | Delete a tunnel | No |
| `wrangler tunnel info <tunnel>` | Display tunnel details | No |
| `wrangler tunnel run [tunnel]` | Run a tunnel via cloudflared (long-running) | No |
| `wrangler tunnel quick-start <url>` | Start free temporary tunnel (long-running) | No |

### 15. Certificates & mTLS

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler cert upload` | Upload a new cert | No |
| `wrangler cert list` | List uploaded mTLS certs | No |
| `wrangler cert delete` | Delete an mTLS cert | No |
| `wrangler mtls-certificate` | Manage mTLS certificates | No |

### 16. Containers (Open Beta)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler containers list` | List containers | No |
| `wrangler containers info <ID>` | Get container info | No |
| `wrangler containers instances <ID>` | List container instances | No |
| `wrangler containers delete <ID>` | Delete a container | No |
| `wrangler containers build <PATH>` | Build a container image | No |
| `wrangler containers push <TAG>` | Push image to registry | No |
| `wrangler containers registries` | Manage non-CF registries | No |
| `wrangler containers images` | Manage images in registry | No |

### 17. AI & AI Search

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler ai models` | List catalog models | No |
| `wrangler ai finetune` | Interact with finetune files | No |
| `wrangler ai-search list` | List AI Search instances | No |
| `wrangler ai-search create <name>` | Create AI Search instance | No |
| `wrangler ai-search get <name>` | Get instance details | No |
| `wrangler ai-search update <name>` | Update instance config | No |
| `wrangler ai-search delete <name>` | Delete instance | No |
| `wrangler ai-search stats <name>` | Get usage statistics | No |
| `wrangler ai-search search <name>` | Execute semantic search | No |

### 18. Secrets Store (Open Beta)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler secrets-store store` | Manage stores | No |
| `wrangler secrets-store secret` | Manage secrets within stores | No |

### 19. Pipelines (Open Beta)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler pipelines list` | List all pipelines | No |
| `wrangler pipelines get <pipeline>` | Get pipeline details | No |
| `wrangler pipelines create <pipeline>` | Create a pipeline | No |
| `wrangler pipelines update <pipeline>` | Update pipeline config | No |
| `wrangler pipelines delete <pipeline>` | Delete a pipeline | No |
| `wrangler pipelines setup` | Interactive pipeline setup | No |
| `wrangler pipelines streams` | Manage pipeline streams | No |
| `wrangler pipelines sinks` | Manage pipeline sinks | No |

### 20. VPC (Open Beta)

| Command | Description | JSON Support |
|---------|-------------|-------------|
| `wrangler vpc service` | Manage VPC services | No |

## JSON Output Support Summary

Commands with explicit `--json` flag:
- `wrangler whoami --json`
- `wrangler d1 list --json`
- `wrangler d1 info <name> --json`
- `wrangler d1 execute <db> --json`
- `wrangler deployments list --json`
- `wrangler versions list --json`
- `wrangler pages project list --json`
- `wrangler pages deployment list --json`
- `wrangler r2 bucket info <bucket> --json`
- `wrangler vectorize list --json`

Commands with `--format json`:
- `wrangler secret list --format json` (default is json)
- `wrangler tail [worker] --format json`

Commands that output JSON by default (no flag needed):
- `wrangler kv namespace list` (outputs JSON array)
- `wrangler kv key list` (outputs JSON array)

## Recommended Scope

### Priority 1 — Core automation (wrap first)

These are the most useful for CI/CD, infrastructure-as-code, and automation scripts:

- **Account**: `whoami` (with JSON)
- **Workers**: `deploy`, `delete` (with dry-run support)
- **Deployments**: `list` (JSON), `status`
- **Versions**: `list` (JSON), `view`, `upload`, `deploy`
- **Secrets**: `list` (JSON), `put`, `delete`, `bulk`
- **D1**: `list` (JSON), `info` (JSON), `create`, `delete`, `execute` (JSON)
- **KV namespace**: `create`, `list` (JSON), `delete`, `rename`
- **KV key**: `put`, `get`, `list` (JSON), `delete`
- **KV bulk**: `get`, `put`, `delete`
- **R2 bucket**: `create`, `list`, `info` (JSON), `delete`
- **R2 object**: `get`, `put`, `delete`

### Priority 2 — Useful automation targets

- **Pages**: `project list` (JSON), `project create`, `project delete`, `deploy`, `deployment list` (JSON), `deployment delete`
- **Queues**: `list`, `create`, `update`, `delete`, `info`, `consumer add/remove`, `pause-delivery`, `resume-delivery`, `purge`
- **D1 migrations**: `create`, `list`, `apply`
- **D1 time-travel**: `info`, `restore`
- **Vectorize**: `list` (JSON), `create`, `delete`, `get`, `info`, `insert`, `upsert`, `query`, `get-vectors`, `delete-vectors`
- **Hyperdrive**: `list`, `create`, `delete`, `get`, `update`
- **Workflows**: `list`, `describe`, `delete`, `trigger`, `instances list/describe/terminate/pause/resume`

### Priority 3 — Lower priority / niche

- **Tunnels**: `list`, `create`, `delete`, `info` (experimental, many users use `cloudflared` directly)
- **Dispatch namespaces**: all commands (Workers for Platforms — niche use case)
- **Containers**: all commands (open beta, rapidly changing)
- **AI / AI Search**: all commands (open beta)
- **Pipelines**: all commands (open beta)
- **Secrets Store**: all commands (open beta)
- **Certificates / mTLS**: all commands
- **VPC**: all commands (open beta)
- **R2 bucket sub-management**: sippy, catalog, notification, domain, dev-url, local-uploads, lifecycle, cors, lock

### Skip — Interactive / TUI / Long-running

- `wrangler login` / `wrangler logout` (browser-based OAuth flow)
- `wrangler dev [script]` (local dev server — long-running process)
- `wrangler tail [worker]` (streaming log session — long-running)
- `wrangler tunnel run` / `wrangler tunnel quick-start` (long-running proxy)
- `wrangler init [name]` (interactive scaffolding with prompts)
- `wrangler setup` (interactive project setup)
- `wrangler pages dev` (local dev server — long-running)
- `wrangler pages deployment tail` (streaming logs — long-running)
- `wrangler pipelines setup` (interactive setup)
- `wrangler docs` (opens browser)
- `wrangler complete` (shell completion generation — utility, not automation)

## JSON Output Examples (from source code and tests)

### `wrangler whoami --json`

**Type**: `WhoamiResult`

Authenticated:
```json
{
  "loggedIn": true,
  "authType": "OAuth Token",
  "email": "user@example.com",
  "accounts": [
    { "name": "Account One", "id": "account-1" },
    { "name": "Account Two", "id": "account-2" },
    { "name": "Account Three", "id": "account-3" }
  ]
}
```

Not authenticated:
```json
{
  "loggedIn": false
}
```

`authType` is one of: `"OAuth Token"`, `"User API Token"`, `"Account API Token"`, `"Global API Key"`.

The command exits with non-zero status if the user is not authenticated.

### `wrangler d1 list --json`

Returns a JSON array of database objects:
```json
[
  {
    "uuid": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
    "name": "my-database",
    "binding": "DB"
  }
]
```

Source type `Database`:
```typescript
type Database = {
  uuid: string;
  previewDatabaseUuid?: string;
  name?: string;
  binding: string;
  internal_env?: string;
  migrationsTableName: string;
  migrationsFolderPath: string;
};
```

Note: The JSON output serializes the full `Database` object from the API. The test shows only `uuid`, `name`, and `binding` because that is what the mock returns. The real API may include additional fields.

### `wrangler d1 info <name> --json`

Returns a single object with database info plus 24h metrics:

Source type `DatabaseInfo`:
```typescript
type DatabaseInfo = {
  uuid: string;
  name: string;
  version: "alpha" | "beta" | "production";
  num_tables: number;
  file_size: number;
  running_in_region?: string;
  read_replication?: {
    mode: "auto" | "disabled";
  };
};
```

The JSON output is a flattened/transformed version:
```json
{
  "name": "my-database",
  "version": "production",
  "num_tables": 5,
  "database_size": "256 kB",
  "running_in_region": "WNAM",
  "read_replication.mode": "auto",
  "read_queries_24h": 1000,
  "write_queries_24h": 50,
  "rows_read_24h": 5000,
  "rows_written_24h": 200
}
```

Notes:
- `uuid` is excluded from output
- `file_size` is converted to human-readable `database_size` via `prettyBytes()`
- `read_replication.mode` is flattened from nested object
- `version` is only included if it equals `"alpha"`
- 24h metrics come from a separate GraphQL query

### `wrangler d1 execute <db> --json`

Returns an array of query results:
```json
[
  {
    "results": [
      { "id": 1, "name": "Alice" },
      { "id": 2, "name": null }
    ],
    "success": true,
    "meta": {
      "duration": 0.52
    }
  }
]
```

For file imports, different structure:
```json
[
  {
    "results": [
      {
        "Total queries executed": 42,
        "Rows read": 100,
        "Rows written": 200,
        "Database size (MB)": "1.5"
      }
    ],
    "success": true,
    "meta": {
      "duration": 123.46
    }
  }
]
```

Notes:
- Outer array contains one entry per SQL statement in a batch
- SQL NULL values serialize as JSON `null`
- `meta.duration` is in milliseconds, formatted to 2 decimal places
- When `--json` is used, logger level is set to `"error"` to prevent banner/warnings from polluting JSON on stdout
- Errors within the batch are returned as `{ "error": {...} }` objects

### `wrangler deployments list --json`

Returns a `DeploymentListResult`:
```json
{
  "latest": {
    "id": "deployment-id-1",
    "number": 3,
    "metadata": {
      "created_on": "2024-01-15T10:30:00Z",
      "author_email": "user@example.com",
      "source": "wrangler"
    },
    "annotations": {
      "workers/triggered_by": "upload",
      "workers/rollback_from": null,
      "workers/message": "Production deploy"
    },
    "resources": {
      "script": { /* handler config */ },
      "bindings": [ /* binding list */ ],
      "script_runtime": {
        "compatibility_date": "2024-01-01",
        "compatibility_flags": [],
        "usage_model": "standard"
      }
    }
  },
  "items": [
    { /* same DeploymentDetails structure */ }
  ]
}
```

`metadata.source` is one of: `"api"`, `"dash"`, `"wrangler"`, `"terraform"`, `"other"`.

### `wrangler versions list --json`

Returns an array of version objects:
```json
[
  {
    "id": "version-id-1",
    "metadata": {
      "created_on": "2024-01-15T10:30:00Z",
      "author_email": "user@example.com",
      "source": "wrangler"
    },
    "annotations": {
      "workers/tag": "v1.2.3",
      "workers/message": "Bug fix release",
      "workers/triggered_by": "upload"
    }
  }
]
```

`annotations["workers/triggered_by"]` is one of: `"upload"`, `"secret"`, `"rollback"`, `"promotion"`.

### `wrangler secret list`

Default format is JSON. Returns array of secret objects:
```json
[
  {
    "name": "API_KEY",
    "type": "secret_text"
  },
  {
    "name": "DATABASE_URL",
    "type": "secret_text"
  }
]
```

With `--format pretty`, outputs human-readable:
```
Secret Name: API_KEY

Secret Name: DATABASE_URL
```

### `wrangler kv namespace list`

Always outputs JSON (no flag needed). Returns array of namespace objects:
```json
[
  {
    "id": "namespace-id-1",
    "title": "MY_KV_NAMESPACE",
    "supports_url_encoding": true
  }
]
```

Source type:
```typescript
interface KVNamespaceInfo {
  id: string;
  title: string;
  supports_url_encoding?: boolean;
}
```

### `wrangler kv key list`

Always outputs JSON (no flag needed). Returns array of key info objects:
```json
[
  {
    "name": "my-key",
    "expiration": 1704067200,
    "metadata": { "created_by": "admin" }
  },
  {
    "name": "another-key"
  }
]
```

Source type:
```typescript
interface NamespaceKeyInfo {
  name: string;
  expiration?: number;
  metadata?: { [key: string]: unknown };
}
```

### `wrangler pages project list --json`

Returns array of formatted project objects:
```json
[
  {
    "Project Name": "my-site",
    "Project Domains": "my-site.pages.dev, custom.example.com",
    "Git Provider": "Yes",
    "Last Modified": "2 hours ago"
  }
]
```

Note: This is a formatted/mapped output, not raw API data. Fields use display-friendly names with spaces, and timestamps are relative ("2 hours ago").

### `wrangler pages deployment list --json`

Returns array of formatted deployment objects:
```json
[
  {
    "Id": "deployment-uuid",
    "Environment": "Production",
    "Branch": "main",
    "Source": "abc1234",
    "Deployment": "https://abc1234.my-site.pages.dev",
    "Status": "2 hours ago",
    "Build": "https://dash.cloudflare.com/account-id/pages/view/my-site/deployment-uuid"
  }
]
```

Notes:
- `Environment` is title-cased: `"Production"` or `"Preview"`
- `Source` is first 7 chars of commit hash
- `Status` is relative time for completed deploys, or build status for in-progress
- These are display-formatted fields, not raw API objects

### `wrangler r2 bucket info <bucket> --json`

Uses `logger.json()` (4-space indentation). Returns:
```json
{
    "name": "my-bucket",
    "created": "2024-01-15T10:30:00Z",
    "location": "WNAM",
    "default_storage_class": "Standard",
    "object_count": "1,234",
    "bucket_size": "5.67 GB"
}
```

Notes:
- `object_count` is locale-formatted (has commas)
- `bucket_size` is human-readable (from prettyBytes)
- Data comes from two API calls: bucket metadata + bucket metrics
- `--json` support was added relatively recently; earlier versions did not have it

### `wrangler vectorize list --json`

Returns array of index objects:
```json
[
  {
    "name": "my-index",
    "description": "Product embeddings",
    "config": {
      "dimensions": 768,
      "metric": "cosine"
    },
    "created_on": "2024-01-15T10:30:00Z",
    "modified_on": "2024-01-20T14:00:00Z"
  }
]
```

Source types:
```typescript
type VectorizeDistanceMetric = "euclidean" | "cosine" | "dot-product";

interface VectorizeIndex {
  name: string;
  description?: string;
  created_on: string;
  modified_on: string;
  config: {
    dimensions: number;
    metric: VectorizeDistanceMetric;
  };
}
```

## Error Handling & Exit Codes

### Exit Codes

Wrangler uses simple exit codes:
- **0**: Success
- **1**: Any error (no differentiation between error types)

There is no granular exit code system. All errors result in `process.exit(1)` or an unhandled exception (which Node defaults to exit code 1).

### Error Output Patterns

Errors are written to **stderr** via `console.error()`. The logger routes:
- `logger.log()` -> `console.log()` -> **stdout**
- `logger.error()` -> `console.error()` -> **stderr**
- `logger.json()` -> `console.log()` -> **stdout** (bypasses log level filtering)

### API Error Format

API errors from the Cloudflare API are wrapped in `APIError` objects. The stderr output looks like:

```
X [ERROR] A request to the Cloudflare API (/accounts/xxx/d1/database) failed.

  the server (api.cloudflare.com) responded with a 404 error:

  {
    "code": 7000,
    "message": "Resource not found"
  }

  If you think this is a bug, please open an issue at:
  https://github.com/cloudflare/workers-sdk/issues/new/choose
```

Common API error codes seen in the wild:
- `7000` - Resource not found
- `7003` - Authentication error / invalid token
- `10000` - Authentication error
- `10007` - Unauthorized

### Authentication Errors

When not authenticated:
```
X [ERROR] You are not currently logged in. Please run `wrangler login`.
```

When token is invalid or expired, the API error format above is used with appropriate error codes.

### Common Error Patterns

**Worker not found** (for secret list, etc.):
```
X [ERROR] Could not find Worker "my-worker". Please deploy before managing secrets.
```

**Missing configuration**:
```
X [ERROR] Missing entry-point: The entry-point should be specified via the command line...
```

**Validation errors** (yargs):
```
Not enough arguments: expected at least 1, received 0
```

### JSON Mode Error Behavior

When `--json` flag is active on commands like `d1 execute`:
- Logger level is set to `"error"` to suppress non-error output from stdout
- Only actual errors and the JSON result appear
- Errors still go to stderr; JSON results go to stdout
- This means you can safely parse stdout for JSON even when errors occur

### Environment Variables for Error Control

- `WRANGLER_LOG=error` - Only show errors
- `WRANGLER_LOG=none` - Suppress all output
- `FORCE_COLOR=0` - Disable ANSI color codes in output (useful for parsing)

## Automation Patterns

### CI/CD Environment Variables

| Variable | Purpose |
|----------|---------|
| `CLOUDFLARE_API_TOKEN` | Authentication (preferred for CI/CD) |
| `CLOUDFLARE_ACCOUNT_ID` | Target account |
| `CLOUDFLARE_ENV` | Target environment (replaces `--env` flag) |
| `CLOUDFLARE_ACCESS_CLIENT_ID` | Access service token client ID |
| `CLOUDFLARE_ACCESS_CLIENT_SECRET` | Access service token secret |
| `WRANGLER_SEND_METRICS=false` | Disable telemetry |
| `FORCE_COLOR=0` | Disable color output |
| `WRANGLER_LOG` | Set log level (`none`, `error`, `warn`, `info`, `log`, `debug`) |

### ND-JSON Output File (for deploy commands)

Set `WRANGLER_OUTPUT_FILE_PATH` or `WRANGLER_OUTPUT_FILE_DIRECTORY` to capture structured output from deploy commands in ND-JSON format. Supported commands:
- `wrangler deploy`
- `wrangler versions upload`
- `wrangler versions deploy`
- `wrangler pages deploy`

Each line in the output file is a separate JSON object containing operation details (worker names, version IDs, deployment URLs, errors).

### Common Scripting Patterns

```bash
# Check if authenticated
if wrangler whoami --json 2>/dev/null | jq -e '.loggedIn' > /dev/null; then
  echo "Authenticated"
fi

# List D1 databases and extract UUIDs
wrangler d1 list --json 2>/dev/null | jq -r '.[].uuid'

# Execute SQL and get results
wrangler d1 execute my-db --command "SELECT * FROM users" --json --remote 2>/dev/null | jq '.[0].results'

# List KV namespaces (already JSON)
wrangler kv namespace list 2>/dev/null | jq -r '.[].id'

# List secrets
wrangler secret list --name my-worker 2>/dev/null | jq -r '.[].name'

# Safe error handling pattern
output=$(wrangler d1 list --json 2>/tmp/wrangler-err)
exit_code=$?
if [ $exit_code -ne 0 ]; then
  echo "Error: $(cat /tmp/wrangler-err)" >&2
  exit 1
fi
echo "$output" | jq .
```

### Key Automation Considerations

1. **stdout vs stderr separation**: JSON output goes to stdout; errors/warnings go to stderr. This is reliable for parsing.
2. **No `--json` on all commands**: Many commands lack JSON output. For those, you must parse human-readable text or use the Cloudflare REST API directly.
3. **Formatted vs raw JSON**: Some commands (Pages project list, Pages deployment list) output display-formatted objects with human-readable field names and relative timestamps, not raw API responses. This makes them less ideal for programmatic use.
4. **KV commands always output JSON**: `kv namespace list` and `kv key list` always produce JSON arrays regardless of flags.
5. **Banner suppression**: When `--json` is used, commands suppress the wrangler banner. Without `--json`, the banner appears on stderr.

## Official Documentation Links

- [Wrangler Commands Reference](https://developers.cloudflare.com/workers/wrangler/commands/)
- [General Commands (whoami, login, etc.)](https://developers.cloudflare.com/workers/wrangler/commands/general/)
- [KV Commands](https://developers.cloudflare.com/workers/wrangler/commands/kv/)
- [KV Commands (KV docs site)](https://developers.cloudflare.com/kv/reference/kv-commands/)
- [R2 Commands](https://developers.cloudflare.com/workers/wrangler/commands/r2/)
- [Wrangler Configuration](https://developers.cloudflare.com/workers/wrangler/configuration/)
- [System Environment Variables](https://developers.cloudflare.com/workers/wrangler/system-environment-variables/)
- [CI/CD Builds Configuration](https://developers.cloudflare.com/workers/ci-cd/builds/configuration/)
- [Wrangler Output File Changelog](https://developers.cloudflare.com/changelog/post/2025-11-03-wrangler-output-file/)
- [GitHub: workers-sdk (wrangler source)](https://github.com/cloudflare/workers-sdk)
- [GitHub Issue: JSON output feature request (#3470)](https://github.com/cloudflare/workers-sdk/issues/3470)
- [GitHub Issue: R2 --json support (#9433)](https://github.com/cloudflare/workers-sdk/issues/9433)
