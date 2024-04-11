## rv
`rv` is a CLI tool for profile-style management of environment variables. If your envrionment is simple, `direnv` is definitely the better option. For more complex environments, `rv` provides ultimate flexibility with a `direnv`-inspired interface.

### Install
```sh
cargo install --git https://github.com/cloud303-cholden/rv.git
```
### Usage
`rv` hooks into your shell and doesn't load environment variables until you explcitily allow it. `rv` looks for an `rv.toml` file in the current directory, and uses the profile passed via `rv set <profile>` to activate an environment. 
### Configuration
The configuration file must be located at `$XDG_CONFIG_HOME/rv/config.toml`. Below is the default configuration:
```toml
[activated]
symbol = "rv ↑ "
style = "green bold"

[activated_dir]
symbol = ""
style = "white"

[deactivated]
symbol = "rv ↓ "
style = "red bold"

[deactivated_dir]
symbol = ""
style = "white"

[added]
symbol = "  "
style = "green bold"

[removed]
symbol = "  "
style = "red bold"

[changed]
symbol = "  "
style = "208 bold"
```
