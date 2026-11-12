# trash

A lightweight, cross-platform trash utility written in Rust.

`trash` moves files and directories to the operating system's Trash or Recycle Bin.

## Features

- Move one or more files and directories to the trash.
- Trash symbolic links while leaving their targets intact.
- Report errors for individual paths and continue processing the remaining paths.
- Reject paths ending in `.` or `..`, and the root directory.

## Installation

### Cargo

```sh
cargo install --git https://github.com/yutadev31/trash
```

### Nix

```sh
nix profile install github:yutadev31/trash
```

## Usage

### Trash a file

```sh
trash file.txt
```

### Trash multiple files and directories

```sh
trash file.txt image.png old-directory
```

Directories and their contents are moved to the trash without requiring a recursive flag.

### Trash a path containing spaces

```sh
trash "old notes.txt"
```

### Trash a path beginning with a hyphen

Use `--` to separate paths from options:

```sh
trash -- -file.txt
```

### Show help

```sh
trash --help
```

`trash` requires at least one path. It reports failures to standard error and exits with a nonzero status if any path fails. Invalid arguments are rejected before any paths are moved.

To restore items or empty the trash, use your desktop environment or operating system's trash interface.

## Platform Support

`trash` uses [trash-rs](https://github.com/Byron/trash-rs) for trash operations.

| Platform | Backend |
|---|---|
| Linux and other supported Unix-like systems | FreeDesktop Trash |
| macOS | Native Trash |
| Windows | Recycle Bin |

On Linux, the default backend follows the FreeDesktop Trash specification, as used by desktop environments such as GNOME, KDE, and XFCE.

## Development

Enter the Nix development environment:

```sh
nix develop
```

Run checks with Cargo:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
```

Run the Nix flake checks:

```sh
nix flake check
```

Build the release binary:

```sh
cargo build --release --locked
```

## License

Licensed under the [MIT License](LICENSE.txt).
