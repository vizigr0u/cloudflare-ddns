# Cloudflare ddns overkill edition

A Dynamic DNS script to update out-of-date Type A DNS entries managed by Cloudflare.

"Overkill edition" because it uses rust but under the hood all it does is a few HTTP calls.

## Getting started

### Option 1: manual install (requires rust)

#### Installing

```sh
cargo install --git https://github.com/vizigr0u/cloudflare-ddns.git
```

#### Running

Set your environment variables and run the script directly.

This is a one-shot script, so you'll need to manage periodic execution yourself (e.g. with a systemd timer).

```sh
export CLOUDFLARE_EMAIL=your-email@example.com
export CLOUDFLARE_API_KEY=your-api-key
export ZONE_ID=your-zone-id
export RECORD_NAMES=my.domain.tld
export IP_PROVIDER_URL=https://api.ipify.org

cloudflare-ddns
```

### Option 2: Docker

The docker image already contains a 1 minute cron so you can just leave it running.

One day I might make the time configurable, feel free to open a pull request.

#### Docker Run

```sh
docker run -d \
  --name dyndns \
  --restart unless-stopped \
  -e CLOUDFLARE_EMAIL=your-email@example.com \
  -e CLOUDFLARE_API_KEY=your-api-key \
  -e ZONE_ID=your-zone-id \
  -e RECORD_NAMES=my.domain.tld \
  -e IP_PROVIDER_URL=https://api.ipify.org \
  ghcr.io/vizigr0u/cloudflare-ddns:0.2.1
```

#### Docker Compose

```yaml
services:
  dyndns:
    image: ghcr.io/vizigr0u/cloudflare-ddns:0.2.1
    container_name: dyndns
    restart: unless-stopped
    env_file:
      - .env
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `CLOUDFLARE_EMAIL` | Your Cloudflare account email address | `example@mail.com` |
| `CLOUDFLARE_API_KEY` | Your Cloudflare API token | `1234567890abcdef1234567890abcdef12345678` |
| `ZONE_ID` | The ID of the Cloudflare zone (domain) you want to update | `1a2b3c4d5e6f7g8h9i0j` |
| `IP_PROVIDER_URL` | The URL of the IP provider to get the current public IP address | `https://api.ipify.org` |
| `RECORD_NAMES` | The name(s) of the DNS record you want to update, separated by colons | `example.com:www.example.com` |

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