##################
### BASE STAGE ###
##################
FROM rust:1.89 as base

ENV TZ=Europe/Stockholm
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y tzdata

WORKDIR /app

# Install required tools
RUN cargo install sqlx-cli

# Install build-target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Required by `brotli-sys`
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev

###################
### BUILD STAGE ###
###################
FROM base as build

# Create a dummy binary for pre-compiling dependencies (for caching)
RUN cargo init . && mkdir -p ./src/bin && mv ./src/main.rs ./src/bin/server.rs
RUN cargo build --release --target x86_64-unknown-linux-musl

# Copy the actual source files
COPY . .

# Compile the final binary
RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl

########################
### PRODUCTION STAGE ###
########################
FROM scratch
WORKDIR /

ENV PORT=8080
EXPOSE 8080

COPY migrations ./
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/server ./hubbit
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

CMD ["./hubbit"]
