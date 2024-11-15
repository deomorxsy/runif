FROM rust:1.82.0-alpine3.20 as rust

RUN <<EOF
apk upgrade && apk update && \
apk add gcc musl-dev rust cargo
EOF

workdir /app
COPY ./native/rust_images ./
COPY ./elixir ./

RUN cargo rustc --release

RUN mix assets.deploy

# Elixir stage
COPY --from=rust /app/target/release/librust_images.so priv/native/librust_images.so

# compile release
RUN mix compile
