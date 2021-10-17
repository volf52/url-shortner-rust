FROM rust as build

WORKDIR /repo

COPY Cargo.lock .
COPY Cargo.toml .
COPY src src

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /repo/target/release/url_shortener /

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["/url_shortener"]