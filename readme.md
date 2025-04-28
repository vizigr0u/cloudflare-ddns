# Cloudflare ddns overkill edition

This script calls the Cloudflare DNS API to update your IP if it has changed since last check.

## Installing

```sh
cargo install --git https://github.com/vizigr0u/cloudflare-ddns.git
```

## Running

### 1. Set environment variables

you can use a `.env` file to specify your environment variables instead of passing them through the script.

See `.env.example`.

### 2. Start

```sh
cloudflare-ddns my.domain.tld other-domain.tld
```

## Why Rust instead of (...)?

Because I like rust.

I also don't think the result looks much worse than most better suited script languages.

## Setting IP_PROVIDER_URL

Any url that responds with only your current IP as raw text will work.

### Examples

- [https://api.ipify.org]
- [https://checkip.amazonaws.com]
- [https://dynamicdns.park-your-domain.com/getip]
- [https://ipv4.icanhazip.com]
