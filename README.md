## totoro
a totp management tool

## Usage

Adding an entry

```sh
totoro add --domain <DOMAIN> --secret <SECRET>
totoro add --domain "github" --secret "xlkasjdqjaqmx"
```

Alternatively, you can directly edit the configuration file:
- Linux: `~/.config/totoro.toml`
- Windows: `{FOLDERID_RoamingAppData}/totoro.toml`
- Mac: `$HOME/Library/Application Support`

Getting a totp value

```sh
totoro get --domain <DOMAIN>
totoro get --domain "github"
```

