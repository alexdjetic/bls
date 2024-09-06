# File Lister

## Overview
File Lister is a command-line tool that lists files and directories with color-coded output. It supports recursive listing and the option to show hidden files.

## Features
- Color-coded output for files and directories.
- Recursive listing with the `-r` flag.
- Option to show hidden files with the `-x` flag.
- Displays file type, owner, group, and permissions.

## Installation

To install the dependencies, run:

```sh
cargo build
```

## Usage

To use the File Lister tool, run the following command:

```bash
bls <PATHS> [OPTIONS]
```

## Options

> -r, --recursive: List directories recursively.

> -x, --hidden: Show hidden files.

## Example

- List files and directories in the current directory:

```bash
bls .
```

- List files and directories recursively:

```bash
bls . -r
```

- List hidden files and directories:

```bash
bls . -x
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Author

Djetic Alexandre - alexandredejtic@proton.me
