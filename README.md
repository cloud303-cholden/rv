## rv
`rv` is a CLI tool for profile-style management of environment variables. If your envrionment is simple, `direnv` is definitely the better option. For more complex environments, `rv` provides ultimate flexibility with a `direnv`-inspired interface.

### Install
```sh
cargo install --git https://github.com/cloud303-cholden/rv.git
```
### Usage
`rv` hooks into your shell and doesn't load environment variables until you explcitily allow it. `rv` looks for an `rv.toml` file in the current directory, and uses the profile passed via `rv set <profile>` to activate an environment. 
