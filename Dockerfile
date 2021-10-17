FROM alpine as upx_temp

WORKDIR /tmp
RUN apk --no-cache add curl tar xz \
    && curl -L https://github.com/upx/upx/releases/download/v3.96/upx-3.96-amd64_linux.tar.xz | tar -xJf - --strip-components 1 upx-3.96-amd64_linux/upx

FROM rust:slim-buster as rust_build

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

FROM node:lts-alpine3.13 as frontend_base

FROM frontend_base as static_deps

WORKDIR /frontend

COPY ./frontend/package.json .
COPY ./frontend/yarn.lock .

RUN yarn install

FROM frontend_base as static_build

WORKDIR /frontend

COPY --from=static_deps /frontend/node_modules node_modules
COPY ./frontend .

RUN yarn build

FROM gcr.io/distroless/cc as release

WORKDIR /app
EXPOSE 80

COPY --from=rust_build /usr/local/cargo/bin/url_shortener /app
COPY --from=static_build /frontend/build/ ./static

COPY Rocket.toml .

CMD ["/app/url_shortener"]