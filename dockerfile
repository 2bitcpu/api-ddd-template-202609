FROM ghcr.io/rust-cross/rust-musl-cross:aarch64-musl AS chef

RUN cargo install cargo-chef

WORKDIR /home/rust/src


FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends upx-ucl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /home/rust/src/recipe.json recipe.json

RUN cargo chef cook \
    --release \
    --target aarch64-unknown-linux-musl \
    --recipe-path recipe.json

COPY . .

RUN cargo build \
    --release \
    --target aarch64-unknown-linux-musl

RUN musl-strip \
    target/aarch64-unknown-linux-musl/release/web-api \
    && upx --best --lzma \
    target/aarch64-unknown-linux-musl/release/web-api \
    && mkdir /html \
    && mkdir /data

    RUN cat << EOF > /html/index.html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Web API Example</title>
</head>
<body>
<h1>Web API Example</h1>
<p>Open the console to see the output.</p>
</body>
</html>
EOF


FROM scratch

COPY --from=builder /home/rust/src/target/aarch64-unknown-linux-musl/release/web-api /web-api
COPY --from=builder /html /html
COPY --from=builder /data /data

EXPOSE 3000

ENTRYPOINT ["/web-api"]
