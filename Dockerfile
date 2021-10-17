FROM alpine as upx_temp

WORKDIR /tmp
RUN apk --no-cache add curl tar xz \
    && curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar -xJf - --strip-components 1 upx-3.96-amd64_linux/upx

FROM rust:slim-buster as build

WORKDIR /repo

COPY Cargo.lock .
COPY Cargo.toml .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN echo "fn main(){}" > dummy.rs
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY --from=upx_temp /tmp/upx ./upx
COPY src src

RUN cargo install --offline --path .
RUN ./upx --best --lzma /usr/local/cargo/bin/url_shortener

FROM gcr.io/distroless/cc

WORKDIR /app
EXPOSE 80

COPY --from=build /usr/local/cargo/bin/url_shortener /app

COPY Rocket.toml .

CMD ["/app/url_shortener"]