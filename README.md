# whiff

[![CICD](https://github.com/mparusinski/whiff/actions/workflows/CICD.yml/badge.svg)](https://github.com/mparusinski/whiff/actions/workflows/CICD.yml)

`whiff` is a reimplementation of `touch` in CLI tool
to change file timestamps and create empty files.

<!-- [Installation](#installation) • [How to use](#how-to-use) • [Troubleshooting](#troubleshooting) -->
[Installation](#installation) • [How to use](#how-to-use)

## How to use

At the moment the only supported functionality to create an empty file.

```bash
whiff new_file
```

## Installation

<!-- TODO: Use repology.org when whiff is distributed in various software distributions channels -->

The software is not yet released in any repositories, it needs to be 
installed by cloning this repo and building the project (see section below).

## Development

```bash
git clone git@github.com:mparusinski/whiff.git

# Build
cd whiff
cargo build

# Run tests
cargo test

# Install
cargo install --path .
```

### Completions

Tab completions for several shells are included in the "autocomplete" directory.
To use these completions put the file in an appropriate location for your shell, and 
depending on your shell, you may need to source the file as well:

- bash: you will need to source the whiff.bash file in your ~/.bashrc file. Or put it in a directory of files that are all sourced.
- zsh: move the "_whiff" file to somewhere on your path
- fish: Put whiff.fish in ~/.config/fish/completions
- powershell: Source the _whiff.ps1 file from one of the files in the [profile scrips locations](https://learn.microsoft.com/en-us/powershell/scripting/learn/shell/creating-profiles?view=powershell-7.5).)

## Maintainers

- [mparusinski](https://github.com/mparusinski)

## License

`whiff` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.

## Acknowledgements

Special thanks for the [`fd` project](github.com/sharkdp/fd) which is a source of inspiration
for this project.
