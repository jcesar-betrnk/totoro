## totoro
a totp management tool

## Prerequisite
- [rust](https://rustup.rs/)

## Installing
```
cargo install --git https://github.com/jcesar-betrnk/totoro
```

## Usage

Adding an entry

```sh
totoro add --domain <DOMAIN> --secret <SECRET>
totoro add --domain "github" --secret "xlkasidioqweqwp"
```

Alternatively, you can directly edit the configuration file:
- Linux: `~/.config/totoro.toml`
- Windows: `{FOLDERID_RoamingAppData}/totoro.toml`
- Mac: `$HOME/Library/Application Support`

The format follows a `key` = `value` and line separated for each entry

Example:
```
github = "xlkasidioqweqwp"
google = "goog ogogo gogole"
```

Getting a totp value

```sh
totoro get --domain <DOMAIN>
totoro get --domain "github"
```

The totp value is valid for 30 seconds and is automatically copied into your clipboard.


## Advance usage
For additional convenience, you can set a local directory in your machine which contains a separate
script for each specific domains

Example:
In your `~/scripts/totp/` directory you'll create a file for each of the domain

```sh
ls -lah
```
```sh
fb.sh
gmail.sh
github.sh
```
Make sure all the script file as an executable permission:

```sh
cd ~/scripts/totp
chmod u+x *.sh
```

```sh
cat github.sh
```

```sh
#!/bin/bash
totoro get --domain "github"
```

Then, you if you need a totp for github. Leveraging autocompletion in the terminal, you only need to type:

`cd sc<TAB>/TO<TAB>` then `./git<TAB>`


