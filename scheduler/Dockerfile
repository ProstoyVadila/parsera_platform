FROM rust:1.77.1-alpine3.19 as builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"

# if needed, add additional dependencies here
RUN apk update && apk add --no-cache musl-dev pkgconfig openssl-dev protobuf-dev
# set the workdir and copy the source into it
WORKDIR /app
COPY scheduler/ /app
COPY common/ /common
# COPY Cargo.toml /app
# COPY . /app
# do a release build
RUN cargo build --release
RUN strip /app/target/release/scheduler

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.19

# if needed, install additional dependencies here
RUN apk add --no-cache libgcc curl
# copy the binary into the final image
# COPY --from=builder /app/Rocket.toml .
COPY --from=builder /app/target/release/scheduler .

# set the binary as entrypoint
ENTRYPOINT ["./scheduler"]
