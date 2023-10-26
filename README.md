<div align="center">

# tangra.rs

[![Crates.io Badge](https://img.shields.io/crates/v/tangra.svg)](https://crates.io/crates/tangra)
[![CI Badge](https://github.com/kkrypt0nn/tangra.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/kkrypt0nn/tangra.rs/actions)
[![Dependency Status Badge](https://deps.rs/repo/github/kkrypt0nn/tangra.rs/status.svg)](https://deps.rs/repo/github/kkrypt0nn/tangra.rs)

[![Discord Server Badge](https://img.shields.io/discord/739934735387721768?logo=discord)](https://discord.gg/mTBrXyWxAF)
[![Last Commit Badge](https://img.shields.io/github/last-commit/kkrypt0nn/tangra.rs)](https://github.com/kkrypt0nn/tangra.rs/commits/main)
[![Conventional Commits Badge](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org/en/v1.0.0/)

</div>

---

A lightweight and easy to use Rust logging library that includes logging functionalities with different levels and
custom
formatting. It can also be used as a library to simply format the various messages you print in the terminal.

## Showcase

### macOS

![macOS Showcase](https://raw.githubusercontent.com/kkrypt0nn/tangra.rs/main/assets/macos.png)

### Linux

![Linux Showcase](https://raw.githubusercontent.com/kkrypt0nn/tangra.rs/main/assets/linux.png)

### Windows

![Windows Showcase](https://raw.githubusercontent.com/kkrypt0nn/tangra.rs/main/assets/windows.png)

## Installation

If you want to use this library for one of your projects, you can install it like any other Go library

```shell
cargo add tangra
```

## Customizing

### Prefix

The prefix, what comes before the message, can be changed with the `set_prefix` method on a `Logger` structure.

> The default prefix is `$[datetime] $[level:color]$[level:name]$[reset]: `

### Placeholders

There are formatting placeholders that will be replaced in both the message and the prefix that can be
seen [here](PLACEHOLDERS.md).

[For example](examples), logging the following message

```
$[fg:red]$[effect:blink]$[effect:bold]$[sys:username] says hello!
```

Will print a red blinking message in bold that says `<username> says hello!`, where `<username>` is the username on your
system.

### Styling

You can choose whether you want to style your messages or not with the `SetStyling` method on a `Logger` structure.
Styling includes foreground colors, background colors and special effects such as bold, and others - see
the [`terminal.rs`](src/terminal.rs) file.

> **Note**: The styling will **not** apply to the message if it is not supported by the terminal.

### Log File

Logs can also be written inside a log file with styling removed. [For example](examples/file.rs):

```rs
fn main() {
    let mut logger = tangra::Logger::new();
    logger.set_log_file_path("example.log");
    logger.debug("$[fg:red]$[effect:blink]$[effect:bold]$[sys:username] says hello!");
}
```

## License

This library was made with 💜 by Krypton and is under the [MIT](LICENSE.md) license.