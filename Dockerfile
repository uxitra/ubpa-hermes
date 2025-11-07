FROM rust:1.91.0-slim-bookworm AS builder

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y musl-tools nodejs npm pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*


RUN rustup target add x86_64-unknown-linux-musl

# Copy everything
COPY . .

RUN ls -l /usr/src/myapp/static

RUN cargo fetch --locked --target x86_64-unknown-linux-musl

RUN cargo install --path .

RUN npm install

# Compile
RUN CARGO_INCREMENTAL=0 \
    RUSTFLAGS="-C strip=debuginfo -C target-feature=+aes,+sse2,+ssse3" \
    cargo build --release --locked --target x86_64-unknown-linux-musl


FROM scratch

# get binary
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-musl/release/ubpa-hermes .
# Get static folder
COPY --from=builder /usr/src/myapp/static ./static/

EXPOSE 8080

CMD ["./ubpa-hermes"]