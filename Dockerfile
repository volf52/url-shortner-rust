FROM alpine as upx_temp

WORKDIR /tmp
RUN apk --no-cache add curl tar xz \
    && curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar -xJf - --strip-components 1 upx-3.96-amd64_linux/upx

FROM rust:slim-buster as build

WORKDIR /repo

COPY dummy.rs .
COPY Cargo.lock .
COPY Cargo.toml .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml \
    && cargo build --release \
    && sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY --from=upx_temp /tmp/upx ./upx
COPY src src

RUN cargo build --release \
    && ./upx --best --lzma /repo/target/release/app

FROM gcr.io/distroless/cc

COPY --from=build /repo/target/release/app /

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/app"]