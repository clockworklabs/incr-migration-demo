# Incremental Migration Demo

This is a [SpacetimeDB](https://spacetimedb.com/) demo project intended to showcase incremental migrations, as described in [How To: Incremental Migrations](https://spacetimedb.com/docs/how-to/incremental-migrations).

This repository has three branches:

- `v1` (current) shows the initial version of the module, before implementing a new feature which requires an added column.
- `fails-publish` shows a version of the module which implements the new feature, but will fail to publish on top of `v1` due to an incompatible schema change.
- `v2` shows a version of the module which implements the new feature, and can cleanly publish on top of `v1` without clearing the database or running a manual migration.
