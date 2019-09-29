# pndev

A CLI tool to aid development @ PN

## Install

* Download the latest release for your platform (Linux or OSX)
from the [releases](https://github.com/PrecisionNutrition/pndev/releases) page.

* copy the binary in your `$PATH` (i.e `sudo cp pndev-linux-amd64 /usr/local/bin/pndev`)

* make it executable `sudo chmod +x /usr/local/bin/pndev`

## Usage

### Help
```
pndev help

# prints

    clone      clone one or all the pn apps
    doctor     diagnose system setup for pndev
    help       Prints this message or the help of the given subcommand(s)
    prepare    run optional setup steps (i.e db setup)
    shell      start a nix-shell in the current application
    start      start ember s or docker + rails - depends on application
    stop       stop docker

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
curl https://nixos.org/nix/install | sh

# add
# . /home/ubuntu/.nix-profile/etc/profile.d/nix.sh
# to your bashrc/zshrc

nix-channel --add https://nixos.org/channels/nixos-19.03 nixos
nix-channel --add https://nixos.org/channels/nixos-19.03 nixpkgs
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

### Cloning

if you do not have the apps already you can use `pndev clone --all`
to clone the most used apps into `~/DEV/PN`

then you can `cd` in one of the supported applications

* [eternal-sledgehammer](https://PrecisionNutrition/eternal-sledgehammer)
* [es-student](https://PrecisionNutrition/es-student)
* [es-certification](https://PrecisionNutrition/es-certification)
* [fitpro](https://PrecisionNutrition/fitpro)
* [payment-next](https://PrecisionNutrition/payment-next)

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

```
pndev prepare
```

This will start the databases, install the dependencies and prepare the database

*NOTE*:

you can safely re-run the `pndev prepare` command every time you want to reset your database


### Start

The `start` command will start the development server for the current application
based on the directory you're in.

Remember that to run the `ember server` you should always have rails running

```
cd ~/DEV/PN/eternal-sledgehammer
pndev start
```


### Shell
To start a development shell that has all the dependencies use

```
pndev shell
```

From here you can use any `bundle` or `yarn` command
remember to prefix commands with `bundle exec` or `yarn exec`


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
