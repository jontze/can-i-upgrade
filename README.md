# can-i-upgrade

`can-i-upgrade` is a CLI tool that helps you investigate if a package in your Node.js project can be upgraded and detects conflicts with peer dependencies.

## Features

- **Check Dependency Upgrades**: Verify if an npm package can be upgraded to a specific version.
- **Analyze Peer Dependencies**: Detect and analyze peer dependencies for compatibility.
- **Detailed Reports**: Generate a report with information on affected packages and their compatibility.

## Installation

To install `can-i-upgrade`, you can use Cargo:

```sh
cargo install --git https://github.com/jontze/can-i-upgrade.git
```

or download the binary from the [releases page](https://github.com/jontze/can-i-upgrade/releases).

## Usage

Run the executable with the `check-dep` in the working directory of your Node.js project.

```sh
can-i-upgrade check-dep --help
Check if you can upgrade the package in your project

Usage: can-i-upgrade check-dep [OPTIONS] <PACKAGE_NAME> <TARGET_VERSION>

Arguments:
  <PACKAGE_NAME>    The name of the npm package to check
  <TARGET_VERSION>  Target Version to check compatibility with

Options:
  -i, --ignore <IGNORE>  Glob patterns to ignore certain dependencies
  -h, --help             Print help
```

### Example

```sh
can-i-upgrade check-dep @angular/core 19.0.0
[1/3] 🔍  Analyze project...
[2/3] 🚚  Collect information of affected dependencies...
[3/3] 📃  Summarize infos...
+-----------------------------------+---------+-------------+-----------------+-----------+
| Package                           | Current | Latest      | Supported Range | Supported |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/animations               | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/common                   | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/compiler                 | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/forms                    | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/platform-browser         | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/platform-browser-dynamic | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| @angular/router                   | 18.2.9  | 19.0.0      | 19.0.0          | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
| jest-preset-angular               | 14.1.0  | 14.4.0-rc.0 | 14.4.0-rc.0     | ✅        |
+-----------------------------------+---------+-------------+-----------------+-----------+
✨  Done in 87 seconds

```

## How It Works

The tool performs the following steps:

1. Analyze Project: Checks if the current directory is an npm project.
2. Collect Information: Gathers information on the specified package and its dependencies.
3. Check Compatibility: Verifies if the target version of the package is compatible with the current project's dependencies.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## Changelog

All notable changes to this project will be documented in the [CHANGELOG](CHANGELOG.md) file.
