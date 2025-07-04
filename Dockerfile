# Build stage
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:latest

LABEL org.opencontainers.image.source=https://github.com/vizigr0u/cloudflare-ddns
LABEL org.opencontainers.image.description="A Dynamic DNS updater for Cloudflare"

# Install cron
RUN apk add --no-cache dcron

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/cloudflare-ddns /usr/local/bin/
RUN chmod +x /usr/local/bin/cloudflare-ddns

# Create directory for cron jobs
RUN mkdir -p /etc/periodic/15min

# Create the cron job script directly (no need for separate file)
RUN echo '#!/bin/sh\n/usr/local/bin/cloudflare-ddns' > /etc/periodic/15min/run-ddns && \
    chmod +x /etc/periodic/15min/run-ddns

# Default environment variables (can be overridden at runtime)
ENV IP_PROVIDER_URL=https://api.ipify.org

# Start crond in the foreground
CMD ["exec", "crond", "-f", "-l", "2"]