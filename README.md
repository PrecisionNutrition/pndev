# pndev

A CLI tool to aid development @ PN

## Install

* Download the latest release for your platform (Linux or OSX)
from the [releases](https://github.com/PrecisionNutrition/pndev/releases) page.

* copy the binary in your `$PATH` (i.e `sudo cp pndev-linux-amd64 /usr/local/bin/pndev`)

* make it executable `sudo chmod +x /usr/local/bin/pndev`

## Configure

If you would like to customize the installation path for the repositories you can create a
`.pndev_config.toml` file in your home directory;

```
# run
pndev doctor
# this will create a ~/.pndev_config.toml for you to edit if you don't have one

```

### Config file format

> the path is relative to your  home directory!

```
install_path = 'DEV/PN'
```


# Usage

For a detailed description of the usage see our private wiki
https://www.notion.so/precisionnutrition/Set-Up-Your-Local-Dev-Environment-bb65105ca8dc475191864d53bfca192f

## List of commands

- [clone](#clone)
- [doctor](#doctor)
- [down](#down)
- [help](#help)
- [prepare](#prepare)
- [ps](#ps)
- [rebuild](#rebuild)
- [reset](#reset)
- [review](#review)
- [shell](#shell)
- [start](#start)
- [stop](#stop)
- [up](#up)
- [update](#update)

### Clone

Clone one repo under the PrecisionNutrition organization
or clone all the main app repos (but not all the addon)

Apps will be cloned into `~/DEV/PN`

then you can `cd` in one of the supported applications

* [eternal-sledgehammer](https://github.com/PrecisionNutrition/eternal-sledgehammer)
* [es-student](https://github.com/PrecisionNutrition/es-student)
* [es-certification](https://github.com/PrecisionNutrition/es-certification)
* [fitpro](https://github.com/PrecisionNutrition/fitpro)
* [payment-next](https://github.com/PrecisionNutrition/payment-next)


#### Usage:

```bash
pndev clone -a
# or
pndev clone es-student
```

### Doctor

first you will want to test that your environment has all the proper setup

```
pndev doctor
```

Install any of the missing applications

#### Git

[Download here](https://git-scm.com/downloads)

#### Docker

[Download Mac](https://docs.docker.com/docker-for-mac/install/)

[Ubuntu](https://docs.docker.com/install/linux/docker-ce/ubuntu/)

on old ubuntu you might have to get a better version of `docker-compose`
https://www.digitalocean.com/community/tutorials/how-to-install-docker-compose-on-ubuntu-16-04

#### Nix

Nix is a cross platform package manager [website](https://nixos.org/nix/)

you can install it with the following commands

```
curl -L https://nixos.org/nix/install | sh

# add
# . /home/ubuntu/.nix-profile/etc/profile.d/nix.sh
# to your bashrc/zshrc

nix-channel --add https://nixos.org/channels/nixos-19.03 nixos
nix-channel --add https://nixos.org/channels/nixos-19.03 nixpkgs
nix-channel --update
```
#### Configuring es-dev domain

Edit your `/etc/hosts` file to point the es-dev domain.

Insert this line (assuming you're running the dev environment locally):

```
127.0.0.1 es-dev.precisionnutrition.com
```

#### Configuring ssh

Ensure you have your `ssh-key` added to your github account

Ideally you should also have your `ssh-agent` running

you can add the keys to the agent by doing

```
ssh-add <path to keyfile>
```

#### Test

run `pndev doctor` again and all checks should be green

### Down

Stops all docker instances
it's equivalent to `docker-compose down` for postsgres, rails etc


#### Usage:

```bash
pndev down
```
### Help

Print help message and gives help on a specific command

#### Usage:

```bash
pndev help
pndev help clone
pndev help ...
```

### Prepare

In order to run the rails server you will need the database credentials from lastpass

Once you have them create a file named `.pn_anonymize_creds` in your `$HOME` directory
and copy the credentials there

```
touch ~/.pn_anonymize_creds
vim ~/.pn_anonymize_creds
```

then `cd` into the rails application

```
cd ~/DEV/PN/eternal-sledgehammer
```

and run the prepare command

```bash
pndev prepare
```

This will start the databases, install the dependencies and prepare the database

*NOTE*:

you can safely re-run the `pndev prepare` command every time you want to reset your database

Optionally you can also pass a `pndev prepare -b` that will restore a bigger database with customer data.

### Ps

Print status of all docker services


#### Usage:

```bash
pndev pd
```

### Rebuild

Rebuild all docker services. This is to be used when the docker configuration changes.

#### Usage:

```bash
pndev rebuild
```

### Reset

When things go wrong `pndev reset` is your friend. There are 3 options

* `deps` Remove all dependecies installed for the current project, gems or node modules. Use this when changing node or ruby versions or when you are worried about your deps being out of date.
* `docker` Resets the docker-compose config for our dev environment. This is rarely needed, but useful when we update postgres or redis or other external services.
* `scratch` This is the thing to use when all else fails. Usually you wanna `cd ~/DEV/PN/eternal-sledgehammer` for this. It will reset everything to master, reset docker and remove all your dependecies. You probably want to follow this with a `pndev prepare`.

#### Usage:

```bash
pndev reset <deps, docker, scratch>
```

### Review

Easily review pull requests, even when they span multiple repositories

Use this command to checkout a specific PR using the branch name



#### Usage:

```bash
pndev review BRANCHNAME

# example
pndev review DS-49

pndev review DS-49 --name fitpro
pndev review DS-49 --name profile-engine
```

output sample

```
ghedamat on lurker in fitpro on  master [⇣$?] is 📦 v0.0.0 via ⬢ v10.17.0 took 1h32m40s
❯ pndev review DS-50
Starting catalog_redis_sidekiq_1 ... done
Starting catalog_postgres_1      ... done
Starting catalog_mailcatcher_1   ... done
Starting catalog_dockerhost_1    ... done
Starting catalog_nginx_1         ... done
⚠ remote branch not found for eternal-sledgehammer:DS-50
⚠ remote branch not found for es-student:DS-50
✓ successfully checked out fitpro:DS-50
⚠ remote branch not found for es-certification:DS-50
⚠ remote branch not found for payment-next:DS-50
```

### Shell

Starts a shell (a `nix-shell` to be precise),
this produces a shell with all the project dependencies (ruby, node, openssl..)

Use this shell to run commands like `bundle exec rails c` or `yarn run ember test --server`

Remember to prefix commands with `bundle exec` or `yarn exec`

#### Usage:

```bash
pndev reset
```

### Start

The `start` command will start the development server for the current application
based on the directory you're in.

Remember that to use one ember app you should always ALSO
have rails running


#### Usage:

```bash
pndev start

# start only docker
pndev start -d
```

### Up

Alias to `pndev start -d`

#### Usage:

```bash
pndev up
```

### Update

Downloads and installs the most recent version of pndev

#### Usage:

```bash
pndev update
```


## Developing

This project uses a nix-shell to provide the build environment

type

```
nix-shell
```

in your console to start.

On first use you will have to install the toolchain

```
rustup toolchain install stable
```

On later uses the basic workflow is

```
cargo run
```

remember that to pass parameter to `pndev`
using `cargo run` you have to use `--`

example

```
cargo run -- check -vvvv
```

### Use clippy

```
cargo clippy --all-targets --all-features -- -D warnings
```

## Releasing
* commit all changes and push
* bump version in `Cargo.toml`
* run `cargo build`
* commit changes to `Cargo.toml` and `Cargo.lock`, name commit X.X.X
* `git tag X.X.X`
* `git push`
* `git push --tags`


## Known Issues

### Killing `pndev start` does not restore command prompt

See https://github.com/PrecisionNutrition/pndev/issues/10 for context

TL;DR;

When killing `pndev start` with CTRL+C press ENTER a couple of times once you see that ember/rails are done with their cleanup.

NOTES:

In order to cleanly return the user to the prompt after killing an ember or rails server we need to wait for the cleanup routines that those processes runs.
Doing so requires adding signal handling and process killing to pndev

We (@ghedamat) decided to punt on this for now.

### Many `pndev` commands fail with `Error: No such file or directory (os error 2)`

If you
- Are on Linux
- Have followed all of the requirement steps
- pndev commands are failing with `Error: No such file or directory (os error 2)`

You might not have the `docker-compose` package installed.

Confirm by running `pndev -v -v -v -v start` for an appropriate amount of troubleshooting verbosity.

This typically applies to Arch/Manjaro/any other distro that isn't Ubuntu.
