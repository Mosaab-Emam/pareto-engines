# Pareto Engines

[![Query Engine](https://github.com/prisma/prisma-engines/actions/workflows/query-engine.yml/badge.svg)](https://github.com/prisma/prisma-engines/actions/workflows/query-engine.yml)
[![Introspection Engine + Migration Engine + sql_schema_describer](https://github.com/prisma/prisma-engines/actions/workflows/migration-engine.yml/badge.svg)](https://github.com/prisma/prisma-engines/actions/workflows/migration-engine.yml)
[![Cargo docs](https://github.com/prisma/prisma-engines/actions/workflows/cargo-doc.yml/badge.svg)](https://github.com/prisma/prisma-engines/actions/workflows/cargo-doc.yml)

**Please note:** This repo started as a fork of [prisma/prisma-engines](https://github.com/prisma/prisma-engines). The objective is to extend & modify it to work with pareto schemas, see [pareto](https://github.com/Mosaab-Emam/pareto).

<hr />

This repository contains a collection of engines that power the core stack for
[Prisma](https://github.com/prisma/prisma), most prominently [Prisma
Client](https://www.prisma.io/client) and [Prisma
Migrate](https://www.prisma.io/migrate).

If you're looking for how to install Prisma or any of the engines, the [Getting
Started](https://www.prisma.io/docs/getting-started) guide might be useful.

This document describes some of the internals of the engines, and how to build
and test them.

## What's in this repository

This repository contains four engines:

- *Query engine*, used by the client to run database queries from Prisma Client
- *Migration engine*, used to create and run migrations
- *Introspection engine*, used to get the state of the database, compare it
    to the schema file, and figure out what the differences are
- *Prisma Format*, used to format prisma files

Additionally, the *psl* (Prisma Schema Language) is the library that defines how
the language looks like, how it's parsed, etc.

You'll also find:
- *libs*, for various (small) libraries such as macros, user facing errors,
    various connector/database-specific libraries, etc.
- a `docker-compose.yml` file that's helpful for running tests and bringing up
    containers for various databases
- a `flake.nix` file for bringing up all dependencies and making it easy to
    build the code in this repository (the use of this file and `nix` is
    entirely optional, but can be a good and easy way to get started)
- an `.envrc` file to make it easier to set everything up, including the `nix
    shell`

## Documentation

The [API docs (cargo doc)](https://prisma.github.io/prisma-engines/) are
published on our fabulous repo page.

## Building Prisma Engines

**Prerequisites:**

- Installed the stable Rust toolchain, at least version 1.52.0. You can get the
  toolchain at [rustup](https://rustup.rs/) or the package manager of your
  choice.
- Linux only: OpenSSL is required to be installed.
- Installed [direnv](https://github.com/direnv/direnv), then `direnv allow` on
  the repository root.
  - Make sure direnv is [hooked](https://direnv.net/docs/hook.html) into your shell
  - Alternatively: Load the defined environment in `./.envrc` manually in your
    shell.
- **For m1 users**: Install [Protocol Buffers](https://grpc.io/docs/protoc-installation/)

Note for nix users: it should be enough to `direnv allow`.
**How to build:**

To build all engines, simply execute `cargo build` on the repository root. This
builds non-production debug binaries. If you want to build the optimized
binaries in release mode, the command is `cargo build --release`.

Depending on how you invoked `cargo` in the previous step, you can find the
compiled binaries inside the repository root in the `target/debug` (without
`--release`) or `target/release` directories (with `--release`):

| Prisma Component     | Path to Binary                                   |
| -------------------- | ------------------------------------------------ |
| Query Engine         | `./target/[debug\|release]/query-engine`         |
| Migration Engine     | `./target/[debug\|release]/migration-engine`     |
| Introspection Engine | `./target/[debug\|release]/introspection-engine` |
| Prisma Format        | `./target/[debug\|release]/prisma-fmt`           |

## Prisma Schema Language

The *Prisma Schema Language* is a library which defines the data structures and
parsing rules for prisma files, including the available database connectors. For
more technical details, please check the [library README](./psl/README.md).

The PSL is used throughout the migration and introspection engines, as well as
prisma format. The DataModeL (DML), which is an annotated version of the PSL is
also used as input for the query engine.

## Query Engine

The *Query Engine* is how Prisma Client queries are executed. Here's a brief
description of what it does:
- takes as inputs an annotated version of the Prisma Schema file called the
    DataModeL (DML),
- using the DML (specifically, the datasources and providers), it builds up a
    [GraphQL](https://graphql.org) model for queries and responses,
- runs as a server listening for GraphQL queries,
- it translates the queries to the respective native datasource(s) and
    returns GraphQL responses, and
- handles all connections and communication with the native databases.

When used through the Prisma Client, there are two ways for the Query Engine to
be executed:
- as a binary, downloaded during installation, launched at runtime;
    communication happens via HTTP (`./query-engine/query-engine`)
- as a native, platform-specific Node.js addon; also downloaded during
    installation (`./query-engine/query-engine-node-api`)

### Usage

You can also run the Query Engine as a stand-alone GraphQL server.

**Warning**: There is no guaranteed API stability. If using it on production
please be aware the api and the query language can change any time.

Notable environment flags:

- `RUST_LOG_FORMAT=(devel|json)` sets the log format. By default outputs `json`.
- `QE_LOG_LEVEL=(info|debug|trace)` sets the log level for the Query Engine. If
    you need Query Graph debugging logs, set it to "trace"
- `FMT_SQL=1` enables logging _formatted_ SQL queries
- `PRISMA_DML_PATH=[path_to_datamodel_file]` should point to the datamodel file
  location. This or `PRISMA_DML` is required for the Query Engine to run.
- `PRISMA_DML=[base64_encoded_datamodel]` an alternative way to provide a
  datamodel for the server.
- `RUST_BACKTRACE=(0|1)` if set to 1, the error backtraces will be printed to
  the STDERR.
- `LOG_QUERIES=[anything]` if set, the SQL queries will be written to the `INFO`
  log. Needs the right log level enabled to be seen from the terminal.
- `RUST_LOG=[filter]` sets the filter for the logger. Can be either `trace`,
  `debug`, `info`, `warning` or `error`, that will output ALL logs from every
  crate from that level. The `.envrc` in this repo shows how to log different
  parts of the system in a more granular way.

Starting the Query Engine:

The engine can be started either with using the `cargo` build tool, or
pre-building a binary and running it directly. If using `cargo`, replace
whatever command that starts with `./query-engine` with `cargo run --bin query-engine --`.

You can also pass `--help` to find out more options to run the engine.

### Metrics

Running `make show-metrics` will start Prometheus and Grafana with a default metrics dashboard.
Prometheus will scrape the `/metrics` endpoint to collect the engine's metrics

Navigate to `http://localhost:3000` to view the Grafana dashboard.

## Migration Engine

The *Migration Engine* does a couple of things:
- creates new migrations by comparing the prisma file with the current state of
    the database, in order to bring the database in sync with the prisma file
- run these migrations and keeps track of which migrations have been executed

The engine uses:
- the prisma files, as the source of truth
- the database it connects to, for diffing and running migrations, as well as
    keeping track of migrations in the `_prisma_migrations` table
- the `prisma/migrations` directory which acts as a database of existing
    migrations

For more information about the migrations engine, check the [crate
README](./migration-engine/README.md).

## Introspection Engine

The *Introspection Engine* is able to (re-)generate a prisma file starting from
a live database.

In a way, it's the opposite of the migration engine: whereas the migration
engine uses the prisma file as the source of truth to update the database, the
introspection engine reverses that dependency. It inspects the database, and
generates a prisma schema file as a result.

For more information about the introspection engine, check the [crate README
](./introspection-engine/README.md).

## Prisma format

Prisma format can format prisma schema files. It also comes as a WASM module via
a node package. You can read more [here](./prisma-fmt-wasm/README.md).

## Debugging

When trying to debug code, here's a few things that might be useful:
- use the language server; being able to go to definition and reason about code
    can make things a lot easier,
- add `dbg!()` statements to validate code paths, inspect variables, etc.,
- you can control the amount of logs you see, and where they come from using the
    `RUST_LOG` environment variable; see [the documentation](https://docs.rs/env_logger/0.9.1/env_logger/#enabling-logging),
- you can use the `test-cli` to test migration and introspection without having
    to go through the `prisma` npm package.

## Testing

There are two test suites for the engines: Unit tests and
integration tests.

- **Unit tests**: They test internal
  functionality of individual crates and components.

  You can find them across the whole codebase, usually in `./tests` folders at
  the root of modules. These tests can be executed via `cargo test`. Note that
  some of them will require the `TEST_DATABASE_URL` enviornment variable set up.

- **Integration tests**: They run GraphQL queries against isolated
  instances of the Query Engine and asserts that the responses are correct.

  You can find them at `./query-engine/connector-test-kit-rs`.

### Set up & run tests:

**Prerequisites:**

- Installed Rust toolchain.
- Installed Docker.
- Installed `direnv`, then `direnv allow` on the repository root.
  - Alternatively: Load the defined environment in `./.envrc` manually in your shell.

**Setup:**
There are helper `make` commands to set up a test environment for a specific
database connector you want to test. The commands set up a container (if needed)
and write the `.test_config` file, which is picked up by the integration
tests:

- `make dev-mysql`: MySQL 5.7
- `make dev-mysql8`: MySQL 8
- `make dev-postgres`: PostgreSQL 10
- `make dev-sqlite`: SQLite
- `make dev-mongodb_5`: MongoDB 5

\*_On windows:_
If not using WSL, `make` is not available and you should just see what your
command does and do it manually. Basically this means editing the
`.test_config` file and starting the needed Docker containers.

To actually get the tests working, read the contents of `.envrc`. Then `Edit environment variables for your account` from Windows settings, and add at least
the correct values for the following variables:

- `WORKSPACE_ROOT` should point to the root directory of `prisma-engines` project.
- `PRISMA_BINARY_PATH` is usually
  `%WORKSPACE_ROOT%\target\release\query-engine.exe`.
- `MIGRATION_ENGINE_BINARY_PATH` should be
  `%WORKSPACE_ROOT%\target\release\migration-engine.exe`.

Other variables may or may not be useful.

**Run:**

Run `cargo test` in the repository root.

## Parallel rust-analyzer builds

When rust-analzyer runs `cargo check` it will lock the build directory and stop any cargo commands from running until it has completed. This makes the build process feel a lot longer. It is possible to avoid this by setting a different build path for
rust-analyzer. To avoid this. Open VSCode settings and search for `Check on Save: Extra Args`. Look for the `Rust-analyzer › Check On Save: Extra Args` settings and add a new directory for rust-analyzer. Something like:

```
--target-dir:/tmp/rust-analyzer-check
```

### Automated integration releases from this repository to npm

(Since July 2022). Any branch name starting with `integration/` will, first, run the full test suite and, second, if passing, run the publish pipeline (build and upload engines to S3)

The journey through the pipeline is the same as a commit on the `main` branch.
- It will trigger [prisma/engines-wrapper](https://github.com/prisma/engines-wrapper) and publish a new [`@prisma/engines-version`](https://www.npmjs.com/package/@prisma/engines-version) npm package but on the `integration` tag.
- Which triggers [prisma/prisma](https://github.com/prisma/prisma) to create a `chore(Automated Integration PR): [...]` PR with a branch name also starting with `integration/`
- Since in prisma/prisma we also trigger the publish pipeline when a branch name starts with `integration/`, this will publish all prisma/prisma monorepo packages to npm on the `integration` tag.
- Our [ecosystem-tests](https://github.com/prisma/ecosystem-tests/) tests will automatically pick up this new version and run tests, results will show in [GitHub Actions](https://github.com/prisma/ecosystem-tests/actions?query=branch%3Aintegration)

This end to end will take minimum ~1h20 to complete, but is completely automated :robot:

Notes:
- in prisma/prisma repository, we do not run tests for `integration/` branches, it is much faster and also means that there is no risk of test failing (e.g. flaky tests, snapshots) that would stop the publishing process.
- in prisma/prisma-engines tests must first pass, before publishing starts. So better keep an eye on them and restart them as needed.


## Security

If you have a security issue to report, please contact us at [security@prisma.io](mailto:security@prisma.io?subject=[GitHub]%20Prisma%202%20Security%20Report%20Engines)
