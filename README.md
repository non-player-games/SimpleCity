# SimpleCity
> To re-create and simplify SimCity.

SimpleCity's goal is to recreate SimCity and simplify its code and
architecture.

## Installing / Getting started

To get started, you will need [Node.js][node] installed

```shell
npm start
```

After you hit `npm start`, it should start an [Electron application](https://electronjs.org/)
up.

## Developing

### Built With

This project is built with Rust on the simulation level and TypeScript
at the user interface level.

Before you start contributing, please be familiar with one of the languages.

### Prerequisites

* [Node.js][node]
* [Rust][rust]

### Setting up Dev

To set up the development environment, simply clone the repository and
install necessary dependencies by running `setup.sh`

```shell
./setup.sh
```

In `setup.sh`, it should install all the dependencies with rust Cargo
and npm.

### Deploying / Publishing

```shell
./deploy.sh
```

> This is still work in progress

## Versioning

We follow [SemVer](http://semver.org/) for versioning. For the versions available, see the [link to tags on this repository](/tags).

## Configuration

> Work in progress

## Tests

The test shell script should execute everything all together.

```shell
./test.sh
```

## Style guide

> Work in progress

## Database

> Work in progress

## Licensing

MIT

[node]: https://nodejs.org/en/
[rust]: https://www.rust-lang.org/en-US/
