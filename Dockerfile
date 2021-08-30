FROM rust:latest as cargo-build

WORKDIR /usr/src/

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/r-meter

COPY . .

RUN cargo build --release

RUN cargo install --path .

FROM alpine:latest

RUN addgroup -g 1000 r-meter

RUN adduser -D -s /bin/sh -u 1000 -G r-meter r-meter

WORKDIR /home/r-meter/bin/

COPY --from=cargo-build /usr/local/cargo/bin/r-meter .

RUN chown r-meter:r-meter r-meter

USER r-meter

CMD ["./r-meter"]
