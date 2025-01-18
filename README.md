## totoro
a totp management tool

## Usage

Adding an entry

```sh
totoro add --domain <DOMAIN> --secret <SECRET>
totoro add --domain "github" --secret "xlkasjdqjaqmx"
```

Getting a totp value

```sh
totoro get --domain <DOMAIN>
totoro get --domain "github"
```
