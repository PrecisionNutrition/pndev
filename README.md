# pndev

A CLI tool to aid development @ PN

## Install

* Download the latest release for your platform (Linux or OSX)
from the [releases](https://github.com/PrecisionNutrition/pndev/releases) page.

* copy the binary in your `$PATH` (i.e `/usr/local/bin`)

## Usage

```
pndev --help
```

first you will want to test that your environment has all the proper setup

```
pndev check
```

then you can `cd` in one of the supported applications

* [eternal-sledgehammer](https://PrecisionNutrition/eternal-sledgehammer)
* [es-student](https://PrecisionNutrition/es-student)
* [es-certification](https://PrecisionNutrition/es-certification)
* [fitpro](https://PrecisionNutrition/fitpro)

and start the development server up with

```
pndev start
```

or start a development shell that has all the dependencies with

```
pndev shell
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


