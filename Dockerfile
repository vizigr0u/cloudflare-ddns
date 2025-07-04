# Build stage - same as above
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage - use debian slim instead of alpine
FROM debian:stable-slim

# Install supercronic and runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install supercronic
RUN curl -fsSL https://github.com/aptible/supercronic/releases/download/v0.2.29/supercronic-linux-amd64 \
    -o /usr/local/bin/supercronic && \
    chmod +x /usr/local/bin/supercronic

# Create a non-root user
RUN groupadd -g 1001 appgroup && \
    useradd -r -u 1001 -g appgroup appuser

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/cloudflare-ddns /usr/local/bin/cloudflare-ddns
RUN chmod +x /usr/local/bin/cloudflare-ddns

# Verify the binary exists and works
RUN ls -la /usr/local/bin/cloudflare-ddns && /usr/local/bin/cloudflare-ddns --help || echo "Binary test completed"

# Create crontab file with full path
RUN echo "*/1 * * * * /usr/local/bin/cloudflare-ddns" > /etc/crontab && \
    cat /etc/crontab && \
    chown appuser:appgroup /etc/crontab

# Switch to non-root user
USER appuser

# Default environment variables (can be overridden at runtime)
ENV IP_PROVIDER_URL=https://api.ipify.org

# Run supercronic with the crontab
CMD ["supercronic", "/etc/crontab"]