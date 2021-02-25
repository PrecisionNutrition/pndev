# Unreleased

- Adds `pndev gh` to open project repo with current branch
- Adds `pndev console` to start a rails console in a nix-shell

# 0.2.0

* `pndev prepare` now defaults to `--quiet` and the old bigger prepare is replaced with `--big`
* `pndev reset` was reworked to have 3 options `deps, docker, scratch`

# 0.1.23

Add `--quiet` option to `pndev prepare`

This new option drops your database and restores a fully migrated new DB with new bootstrap data but does NOT download any data from the cloud.
This means that it's VERY FAST, but also provides no program data.

It's useful if you are not working on things that require Activities, Intakes and other program data to be present.
