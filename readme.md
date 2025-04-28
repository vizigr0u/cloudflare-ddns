# Cloudflare ddns overkill edition

This script calls the Cloudflare DNS API to update your IP if it has changed since last check.

## Why Rust instead of (...)?

Because I like rust.

I also don't think the result looks much worse than most better suited script languages.

## Usage

### Simple one-shot use

```sh
CLOUDFLARE_EMAIL=... \
CLOUDFLARE_API_KEY=... \
ZONE_ID=... \
DNS_RECORD_ID=... \
cloudflare-ddns
```

### Support for .env

you can use a .env file to specify your environment variables instead of passing them through the script

```sh
cp .env.example .env
# (edit .env)
cloudflare-ddns
```

## Examples IP providers

- [https://checkip.amazonaws.com]
- [https://dynamicdns.park-your-domain.com/getip]
- [https://ipv4.icanhazip.com]
